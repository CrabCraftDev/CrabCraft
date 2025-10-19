use crate::blocks::Block;
use crate::chunks::bit_storage::BitStorage;
use crate::chunks::chunk::Chunk;
use crate::chunks::palette::palette_content_type::PaletteContentType;
use crate::chunks::palette::paletted_storage::Palette;
use crate::chunks::palette::paletted_storage::PalettedStorage;
use crate::chunks::section::ChunkSection;
use crate::error::AnvilError;
use crate::error::AnvilError::BlockEntityDoesNotExist;
use crab_nbt::NbtCompound;
use crab_protocol::packets::play::ChunkDataBlockEntity;
use crab_registry::REGISTRY;
use crab_registry::minecraft::BlockEntity;
use crab_text::utils::namespaced_key::NamespacedKey;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;

pub const ANVIL_DATA_VERSION: i32 = 4435;

/// Represents a Minecraft chunk in the Anvil file format.
/// https://minecraft.wiki/w/Chunk_format
#[derive(Debug, Serialize, Deserialize)]
pub struct AnvilChunk {
    #[serde(rename = "DataVersion")]
    pub data_version: i32,
    #[serde(rename = "xPos")]
    pub x_pos: i32,
    #[serde(rename = "zPos")]
    pub z_pos: i32,
    #[serde(rename = "yPos")]
    pub y_pos: i32,
    #[serde(rename = "Status")]
    pub status: NamespacedKey,
    #[serde(rename = "LastUpdate")]
    pub last_update: i64,
    sections: Vec<RawSection>,
    #[serde(rename = "block_entities")]
    block_entities: Option<Vec<AnvilBlockEntity>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RawSection {
    #[serde(rename = "Y")]
    pub y: i8,
    pub block_states: PaletteData<BlockStatePaletteEntry>,
    pub biomes: PaletteData<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PaletteData<T> {
    pub palette: Vec<T>,
    pub data: Option<Vec<i64>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AnvilBlockEntity {
    pub id: NamespacedKey,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    #[serde(flatten)]
    pub extra: NbtCompound,
}

#[derive(Debug, Serialize, Deserialize)]
struct BlockStatePaletteEntry {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Properties")]
    pub properties: Option<HashMap<String, String>>,
}

impl AnvilChunk {
    pub fn to_chunk(self, get_biome: impl Fn(&str) -> u64) -> Result<Chunk, AnvilError> {
        Ok(Chunk {
            sections: Self::load_sections(self.sections, get_biome)?,
            block_entities: Self::load_block_entities(self.block_entities.unwrap_or_default())?,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn from_chunk(
        chunk: Chunk,
        get_biome_name: impl Fn(u64) -> String,
        x_pos: i32,
        z_pos: i32,
        y_pos: i32,
        data_version: i32,
        status: NamespacedKey,
        last_update: i64,
    ) -> Result<Self, AnvilError> {
        let sections = chunk
            .sections
            .into_iter()
            .enumerate()
            .map(|(y_index, section)| RawSection::from_chunk_section(y_index as i8, section, &get_biome_name))
            .collect::<Result<Vec<_>, _>>()?;

        let root_registry = REGISTRY.load();
        let registry = root_registry.get_registry(BlockEntity::REGISTRY).unwrap();

        let block_entities = chunk
            .block_entities
            .into_iter()
            .map(|entity| {
                let key = registry.id_to_key(entity.block_entity_type as usize).unwrap().clone();

                Ok(AnvilBlockEntity {
                    id: key,
                    x: x_pos * 16 + entity.relative_x as i32,
                    y: entity.y as i32,
                    z: z_pos * 16 + entity.relative_z as i32,
                    extra: entity.data.into(),
                })
            })
            .collect::<Result<Vec<_>, AnvilError>>()?;

        Ok(Self {
            data_version,
            x_pos,
            z_pos,
            y_pos,
            status,
            last_update,
            sections,
            block_entities: Some(block_entities),
        })
    }

    fn load_sections(
        sections: Vec<RawSection>,
        get_biome: impl Fn(&str) -> u64,
    ) -> Result<Vec<ChunkSection>, AnvilError> {
        sections
            .into_iter()
            .map(|section| section.into_chunk_section(&get_biome))
            .collect()
    }

    fn load_block_entities(block_entities: Vec<AnvilBlockEntity>) -> Result<Vec<ChunkDataBlockEntity>, AnvilError> {
        block_entities
            .into_iter()
            .map(|entity| {
                let root_registry = REGISTRY.load();
                let registry = root_registry.get_registry(BlockEntity::REGISTRY).unwrap();

                let numeric_id = registry
                    .key_to_id(&entity.id)
                    .ok_or(BlockEntityDoesNotExist(entity.id))?;

                Ok(ChunkDataBlockEntity {
                    relative_x: (entity.x % 16) as i8,
                    relative_z: (entity.z % 16) as i8,
                    y: entity.y as i16,
                    block_entity_type: numeric_id as i32,
                    data: entity.extra.into(),
                })
            })
            .collect()
    }
}

impl RawSection {
    fn into_chunk_section(self, get_biome: &impl Fn(&str) -> u64) -> Result<ChunkSection, AnvilError> {
        let biome_id_getter = |name: &String| Ok(get_biome(name) as u32);
        let block_id_getter = |entry: &BlockStatePaletteEntry| {
            let props = entry
                .properties
                .as_ref()
                .map(|p| p.iter().map(|(k, v)| (k.as_str(), v.clone())).collect())
                .unwrap_or_default();
            Ok(Block::from_properties(&NamespacedKey::from_str(&entry.name)?, &props)?.0 as u32)
        };

        let biomes_storage = self.biomes.into_palette(biome_id_getter, PaletteContentType::Biomes)?;
        let blocks_storage = self
            .block_states
            .into_palette(block_id_getter, PaletteContentType::Blocks)?;

        Ok(ChunkSection::new(blocks_storage, biomes_storage))
    }
}

impl<T> PaletteData<T> {
    fn into_palette(
        self,
        to_id: impl Fn(&T) -> Result<u32, AnvilError>,
        content_type: PaletteContentType,
    ) -> Result<PalettedStorage, AnvilError> {
        let data = self.data.unwrap_or_default();
        let storage = BitStorage::new(content_type.amount_of_entries(), data);

        let palette: Vec<u32> = self.palette.iter().map(to_id).collect::<Result<Vec<_>, AnvilError>>()?;

        Ok(PalettedStorage::new(
            content_type,
            Palette::new(palette, storage, &content_type),
        ))
    }
}

impl RawSection {
    fn from_chunk_section(
        y: i8,
        section: ChunkSection,
        get_biome_name: &impl Fn(u64) -> String,
    ) -> Result<Self, AnvilError> {
        let (block_palette, block_data) = section.blocks.into_parts();
        let (biome_palette, biome_data) = section.biomes.into_parts();

        let block_palette = block_palette
            .into_iter()
            .map(|id| {
                let block = Block(id as u16);
                let (key, props) = block.properties()?;
                Ok(BlockStatePaletteEntry {
                    name: key.to_string(),
                    properties: Some(props.into_iter().map(|(k, v)| (k.to_string(), v)).collect()),
                })
            })
            .collect::<Result<Vec<_>, AnvilError>>()?;

        let biome_palette = biome_palette
            .into_iter()
            .map(|id| Ok(get_biome_name(id as u64)))
            .collect::<Result<Vec<_>, AnvilError>>()?;

        Ok(Self {
            y,
            block_states: PaletteData {
                palette: block_palette,
                data: block_data.map(|data| data.data),
            },
            biomes: PaletteData {
                palette: biome_palette,
                data: biome_data.map(|data| data.data),
            },
        })
    }
}
