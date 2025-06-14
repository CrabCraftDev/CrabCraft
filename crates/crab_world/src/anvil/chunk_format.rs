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
        raw_sections: Vec<RawSection>,
        get_biome: impl Fn(&str) -> u64,
    ) -> Result<Vec<ChunkSection>, AnvilError> {
        let mut sections = Vec::with_capacity(raw_sections.len());
        for section in raw_sections {
            let blocks_storage = section.block_states.into_palette(
                |entry| {
                    let properties = entry
                        .properties
                        .as_ref()
                        .map(|props| props.iter().map(|(k, v)| (k.as_str(), v.clone())).collect())
                        .unwrap_or_default();

                    Ok(Block::from_properties(&NamespacedKey::from_str(&entry.name)?, &properties)?.0 as u32)
                },
                PaletteContentType::Blocks,
            )?;

            let biomes_storage = section
                .biomes
                .into_palette(|entry| Ok(get_biome(entry) as u32), PaletteContentType::Biomes)?;

            sections.push(ChunkSection::new(blocks_storage, biomes_storage));
        }
        Ok(sections)
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

impl<T> PaletteData<T> {
    fn into_palette(
        self,
        translate: impl Fn(&T) -> Result<u32, AnvilError>,
        content_type: PaletteContentType,
    ) -> Result<PalettedStorage, AnvilError> {
        let biomes_states_values = self.data.unwrap_or_default();
        let storage = BitStorage::new(content_type.amount_of_entries(), biomes_states_values);

        let palette: Vec<u32> = self
            .palette
            .iter()
            .map(translate)
            .collect::<Result<Vec<_>, AnvilError>>()?;

        Ok(PalettedStorage::new(
            content_type,
            Palette::new(palette, storage, &content_type),
        ))
    }
}
