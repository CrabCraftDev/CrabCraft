use crate::chunks::paletted_container::get_entries_as_longs;
use crate::chunks::section::ChunkSection;
use crate::utils::errors::Error;
use bytes::BytesMut;
use crab_nbt::NbtTag;

pub const CHUNK_SECTIONS_PER_WORLD_HEIGHT: usize = 24;
pub const AMOUNT_OF_BLOCKS_IN_CHUNK_SECTION: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;
pub const AMOUNT_OF_BIOMES_IN_CHUNK_SECTION: usize = 4 * 4 * 4;
pub const CHUNK_SIZE: usize = 16;

/// Chunk is a collection of blocks with a horizontal size of 16×16, spanning the entire buildable area on the vertical axis.
/// Each chunk consists of [sections](ChunkSection) - a 16×16×16 cube of blocks.
#[derive(Debug, Clone)]
pub struct Chunk {
    pub sections: Vec<ChunkSection>,
}

impl Chunk {
    fn get_block(&self, x: u32, y: u32, z: u32) -> u16 {
        let index = x + z * CHUNK_SIZE as u32 + y % AMOUNT_OF_BLOCKS_IN_CHUNK_SECTION as u32;
        *self.sections[y as usize / CHUNK_SIZE]
            .blocks
            .get(index as usize)
            .unwrap()
    }

    /// Calculates the [heightmap](https://wiki.vg/Chunk_Format#Heightmaps_structure) of the chunk.
    /// The world surface heightmap is a 16×16 array of u16 values, where each value represents the height of the highest non-air block at that position.
    pub fn get_world_surface_height_map(&self) -> Vec<u16> {
        let mut heightmap = vec![0; 16 * 16];
        for z in 0..16 {
            for x in 0..16 {
                for y in (0..(CHUNK_SECTIONS_PER_WORLD_HEIGHT * 16)).rev() {
                    let block = self.get_block(x, y as u32, z);
                    if block != 0 {
                        heightmap[(z * 16 + x) as usize] = y as u16;
                        break;
                    }
                }
            }
        }
        heightmap
    }

    pub fn serialize_world_surface_height_map(&self) -> NbtTag {
        const BITS_PER_ENTRY: u32 = 9;

        let heightmap: Vec<i32> = self.get_world_surface_height_map().iter().map(|&v| v as i32).collect();
        let encoded_heightmap = get_entries_as_longs(&heightmap, BITS_PER_ENTRY as usize);

        NbtTag::LongArray(encoded_heightmap)
    }

    /// Serializes the section to format used in [Chunk Data packet](crab_protocol::packets::play::ChunkDataAndUpdateLightS2CPacket).
    /// For more information about the format, see [wiki.vg/Chunk_Format](https://wiki.vg/Chunk_Format).
    pub fn serialize_to_mc_format(&mut self, chunk: &mut BytesMut) -> Result<(), Error> {
        for section in &mut self.sections {
            section.serialize_to_mc_format(chunk)?;
        }
        Ok(())
    }
}

impl Default for Chunk {
    fn default() -> Self {
        let sections = vec![ChunkSection::default(); CHUNK_SECTIONS_PER_WORLD_HEIGHT];
        Chunk { sections }
    }
}
