use crate::chunks::chunk::{AMOUNT_OF_BIOMES_IN_CHUNK_SECTION, AMOUNT_OF_BLOCKS_IN_CHUNK_SECTION};
use crate::chunks::paletted_container::{BIOMES_CONTAINER, BLOCKS_CONTAINER};
use crate::utils::errors::Error;
use bytes::{BufMut, Bytes, BytesMut};

/// Chunk section is a 16×16×16 collection of blocks.
/// It stores blocks and biomes in a [paletted](PalettedContainer) format.
#[derive(Debug, Clone)]
pub struct ChunkSection {
    pub non_air_block_amount: u16,
    pub blocks: [u16; AMOUNT_OF_BLOCKS_IN_CHUNK_SECTION],
    pub biomes: [u8; AMOUNT_OF_BIOMES_IN_CHUNK_SECTION],
    pub cached_bytes: Option<Bytes>,
}

impl ChunkSection {
    pub fn new(
        blocks: [u16; AMOUNT_OF_BLOCKS_IN_CHUNK_SECTION],
        biomes: [u8; AMOUNT_OF_BIOMES_IN_CHUNK_SECTION],
    ) -> Self {
        Self {
            non_air_block_amount: blocks.iter().filter(|&v| *v != 0).count() as u16,
            blocks,
            biomes,
            cached_bytes: None,
        }
    }

    pub fn set(&mut self, x: u8, y: u8, z: u8, block: u16) {
        self.cached_bytes = None;

        let index = self.calculate_index(x, y, z);
        let previous_block = self.blocks[index];

        if previous_block == 0 && block != 0 {
            self.non_air_block_amount += 1;
        } else if previous_block != 0 && block == 0 {
            self.non_air_block_amount -= 1;
        }

        self.blocks[index] = block;
    }

    #[allow(dead_code)]
    pub fn get(&mut self, x: u8, y: u8, z: u8) -> u16 {
        self.blocks[self.calculate_index(x, y, z)]
    }

    pub fn serialize_to_mc_format(&mut self, chunk_section: &mut BytesMut) -> Result<(), Error> {
        if let Some(cached_bytes) = &self.cached_bytes {
            chunk_section.put_slice(cached_bytes);
            return Ok(());
        }

        let mut bytes = BytesMut::new();
        bytes.put_i16(self.non_air_block_amount as i16);
        BLOCKS_CONTAINER.serialize_palette(self.blocks.iter().map(|&v| v as i32).collect(), &mut bytes)?;
        BIOMES_CONTAINER.serialize_palette(self.biomes.iter().map(|&v| v as i32).collect(), &mut bytes)?;

        chunk_section.put_slice(&bytes);
        self.cached_bytes = Some(bytes.freeze());

        Ok(())
    }

    fn calculate_index(&self, x: u8, y: u8, z: u8) -> usize {
        (y as usize * 16 * 16) + (z as usize * 16) + x as usize
    }
}

impl Default for ChunkSection {
    fn default() -> Self {
        const PLAINS_BIOME_ID: u8 = 39;
        Self::new(
            [0; AMOUNT_OF_BLOCKS_IN_CHUNK_SECTION],
            [PLAINS_BIOME_ID; AMOUNT_OF_BIOMES_IN_CHUNK_SECTION],
        )
    }
}
