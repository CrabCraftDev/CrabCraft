use crate::blocks::Block;
use crate::chunks::bit_storage::BitStorage;
use crate::chunks::palette::paletted_storage::calculate_bits_per_entry;
use crate::chunks::section::CHUNK_SIZE_SHIFT;
use crate::chunks::section::{CHUNK_SIZE, ChunkSection};
use crate::error::WorldError;
use bytes::BytesMut;
use crab_nbt::NbtTag;
use crab_protocol::packets::play::ChunkDataBlockEntity;

/// The number of chunk sections that fit in the standard overworld height (384 blocks)
pub const CHUNK_SECTIONS_PER_WORLD_HEIGHT: usize = 24;

/// Chunk is a collection of blocks with a horizontal size of 16×16, spanning the entire buildable area on the vertical axis.
/// Each chunk consists of [sections](ChunkSection) - a 16×16×16 cube of blocks.
/// Block entities are blocks that have additional data associated with them, such as chests, furnaces, and signs.
#[derive(Debug, Clone)]
pub struct Chunk {
    pub sections: Vec<ChunkSection>,
    pub block_entities: Vec<ChunkDataBlockEntity>,
}

impl Chunk {
    pub fn without_sections() -> Chunk {
        Chunk {
            sections: Vec::new(),
            block_entities: Vec::new(),
        }
    }

    pub fn get_block(&self, x: u8, y: u16, z: u8) -> Option<Block> {
        let section_index = (y >> CHUNK_SIZE_SHIFT) as usize;
        let local_y = (y & (CHUNK_SIZE - 1)) as u8;
        self.sections[section_index].get_block(x, local_y, z)
    }

    pub fn set_block(&mut self, x: u8, y: u16, z: u8, block: Block) -> Result<(), WorldError> {
        let section_index = (y >> CHUNK_SIZE_SHIFT) as usize;
        let local_y = (y & (CHUNK_SIZE - 1)) as u8;
        self.sections[section_index].set_block(x, local_y, z, block)
    }

    /// Serializes the section to format used in [Chunk Data packet](crab_protocol::packets::play::ChunkDataAndUpdateLightS2CPacket).
    /// For more information about the format, see [Minecraft Wiki](https://minecraft.wiki/w/Chunk_format).
    pub fn serialize_to_mc_format(&mut self, chunk: &mut BytesMut) -> Result<(), WorldError> {
        for section in &mut self.sections {
            section.serialize_to_mc_format(chunk)?;
        }
        Ok(())
    }

    pub fn serialize_world_surface_height_map(&self) -> NbtTag {
        let height_map_bits_per_entry = calculate_bits_per_entry(self.world_height() + 1);

        let heightmap: Vec<i64> = self.get_world_surface_height_map().iter().map(|&v| v as i64).collect();
        let encoded_heightmap = BitStorage::from_iter(heightmap, height_map_bits_per_entry);

        NbtTag::LongArray(encoded_heightmap.data)
    }

    /// Calculates the [heightmap](https://web.archive.org/web/20241129083318/https://wiki.vg/Chunk_Format#Heightmaps_structure) of the chunk.
    /// The world surface heightmap is a 16×16 array of u16 data, where each value represents the height of the highest non-air block at that position.
    pub fn get_world_surface_height_map(&self) -> Vec<u16> {
        let mut heightmap = vec![0; 16 * 16];
        for z in 0..16 {
            for x in 0..16 {
                for y in (0..self.world_height()).rev() {
                    let block = self.get_block(x, y as u16, z);
                    if let Some(block) = block
                        && !block.is_air()
                    {
                        heightmap[(z * 16 + x) as usize] = y as u16;
                        break;
                    }
                }
            }
        }
        heightmap
    }

    fn world_height(&self) -> usize {
        self.sections.len() * 16
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Chunk {
            sections: vec![ChunkSection::default(); CHUNK_SECTIONS_PER_WORLD_HEIGHT],
            block_entities: Vec::new(),
        }
    }
}
