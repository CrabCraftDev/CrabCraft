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
use crab_protocol::packets::play::BlockEntity;
use crab_registry::RegistryManager;
use crab_text::utils::namespaced_key::NamespacedKey;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;

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
    pub status: String,
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
    pub fn to_chunk(self, get_biome: impl Fn(&str) -> u64, registry: &RegistryManager) -> Result<Chunk, AnvilError> {
        Ok(Chunk {
            sections: Self::load_sections(self.sections, get_biome)?,
            block_entities: Self::load_block_entities(self.block_entities.unwrap_or_default(), registry)?,
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

    fn load_block_entities(
        block_entities: Vec<AnvilBlockEntity>,
        registry: &RegistryManager,
    ) -> Result<Vec<BlockEntity>, AnvilError> {
        block_entities
            .into_iter()
            .map(|entity| {
                let numeric_id = registry
                    .block_entities
                    .get(&entity.id)
                    .ok_or(BlockEntityDoesNotExist(entity.id))?;

                Ok(BlockEntity {
                    relative_x: (entity.x % 16) as i8,
                    relative_z: (entity.z % 16) as i8,
                    y: entity.y as i16,
                    block_entity_type: *numeric_id as i32,
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
