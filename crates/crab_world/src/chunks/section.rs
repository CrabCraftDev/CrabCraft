use crate::blocks::Block;
use crate::chunks::palette::palette_content_type::PaletteContentType;
use crate::chunks::palette::paletted_storage::{Palette, PalettedStorage};
use crate::error::WorldError;
use bytes::{BufMut, BytesMut};
use crab_registry::REGISTRY;
use crab_registry::vanilla::Biome;
use std::fmt::Debug;

pub const CHUNK_SIZE: u16 = 16;
pub const CHUNK_SIZE_SHIFT: u32 = CHUNK_SIZE.trailing_zeros();
pub const AMOUNT_OF_BIOMES_IN_CHUNK_SECTION: u16 = 4 * 4 * 4;
pub const AMOUNT_OF_BLOCKS_IN_CHUNK_SECTION: usize = CHUNK_SIZE as usize * CHUNK_SIZE as usize * CHUNK_SIZE as usize;

/// Chunk section is a 16×16×16 collection of blocks.
/// It stores blocks and biomes in a [paletted](crate::paletted_container::PalettedContainer) format.
#[derive(Debug, Clone, PartialEq)]
pub struct ChunkSection {
    pub non_air_block_amount: u16,
    pub blocks: PalettedStorage,
    pub biomes: PalettedStorage,
}

impl ChunkSection {
    pub fn new(blocks: PalettedStorage, biomes: PalettedStorage) -> Self {
        let mut value = Self {
            non_air_block_amount: 0,
            blocks,
            biomes,
        };
        value.count_non_air_blocks();
        value
    }

    pub fn count_non_air_blocks(&mut self) {
        let is_not_air = |block: i64| !Block::is_air_i64(block);
        self.non_air_block_amount = match &self.blocks.palette {
            Palette::Single(value) => {
                if Block(*value as u16).is_air() {
                    0
                } else {
                    AMOUNT_OF_BLOCKS_IN_CHUNK_SECTION
                }
            }
            Palette::Indirect {
                values_lookup_table,
                storage,
            } => {
                let has_air = values_lookup_table.iter().any(|&value| Block::is_air_i64(value as i64));
                if has_air {
                    storage.count(is_not_air)
                } else {
                    AMOUNT_OF_BLOCKS_IN_CHUNK_SECTION
                }
            }
            Palette::Direct(storage) => storage.count(is_not_air),
        } as u16;
    }

    pub fn serialize_to_mc_format(&self, chunk_section: &mut BytesMut) -> Result<(), WorldError> {
        chunk_section.put_i16(self.non_air_block_amount as i16);
        self.blocks.serialize(chunk_section);
        self.biomes.serialize(chunk_section);
        Ok(())
    }

    pub fn set_block(&mut self, x: u8, y: u8, z: u8, block: Block) -> Result<(), WorldError> {
        let index = self.calculate_index(x, y, z)?;
        self.update_non_air_block_count(x, y, z, block)?;

        self.blocks.set_at_index(index, block.0 as u32)
    }

    pub fn get_block(&self, x: u8, y: u8, z: u8) -> Option<Block> {
        let index = self.calculate_index(x, y, z).ok()?;

        self.blocks.get_at_index(index).map(|block_id| Block(block_id as u16))
    }

    fn update_non_air_block_count(&mut self, x: u8, y: u8, z: u8, new: Block) -> Result<(), WorldError> {
        if let Some(old) = self.get_block(x, y, z) {
            if old.is_air() && !new.is_air() {
                self.non_air_block_amount += 1;
            } else if !old.is_air() && new.is_air() {
                self.non_air_block_amount -= 1;
            }
            Ok(())
        } else {
            Err(WorldError::MissingBlock(x, y, z))
        }
    }

    fn calculate_index(&self, x: u8, y: u8, z: u8) -> Result<usize, WorldError> {
        if x > 15 || y > 15 || z > 15 {
            return Err(WorldError::InvalidPositionInChunkSection(x, y, z));
        }

        Ok((y as usize * 16 * 16) + (z as usize * 16) + x as usize)
    }
}

impl Default for ChunkSection {
    fn default() -> Self {
        let registry = REGISTRY.load();
        let plains_biome_id = registry.key_to_id(Biome::PLAINS) as u32;

        ChunkSection {
            non_air_block_amount: 0,
            blocks: PalettedStorage::single(PaletteContentType::Blocks, 0),
            biomes: PalettedStorage::single(PaletteContentType::Biomes, plains_biome_id),
        }
    }
}
