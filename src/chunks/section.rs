use crate::chunks::chunk::{AMOUNT_OF_BIOMES_IN_CHUNK_SECTION, AMOUNT_OF_BLOCKS_IN_CHUNK_SECTION};
use crate::chunks::paletted_container::{BIOMES_CONTAINER, BLOCKS_CONTAINER};
use crate::utils::errors::Error;
use crate::REGISTRY;
use bytes::{BufMut, Bytes, BytesMut};
use crab_protocol::types::identifier::Identifier;
use crab_registry::blocks::{get_block, Air, Block};

/// Chunk section is a 16×16×16 collection of blocks.
/// It stores blocks and biomes in a [paletted](PalettedContainer) format.
#[derive(Debug, Clone)]
pub struct ChunkSection {
    pub non_air_block_amount: u16,
    blocks: [u16; AMOUNT_OF_BLOCKS_IN_CHUNK_SECTION],
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

    pub fn set_block(&mut self, x: u8, y: u8, z: u8, block: &dyn Block) {
        self.cached_bytes = None;

        let index = self.calculate_index(x, y, z);
        let previous_block = get_block(self.blocks[index]).unwrap();

        if Air::is_same_type_as_self(&*previous_block) && !Air::is_same_type_as_self(block) {
            self.non_air_block_amount += 1;
        } else if !Air::is_same_type_as_self(&*previous_block) && Air::is_same_type_as_self(block) {
            self.non_air_block_amount -= 1;
        }

        self.blocks[index] = block.get_state_id();
    }

    #[allow(dead_code)]
    pub fn get_block(&self, x: u8, y: u8, z: u8) -> Option<Box<dyn Block>> {
        get_block(self.blocks[self.calculate_index(x, y, z)])
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
        let registry = &REGISTRY.read().unwrap();
        let biome_registry_identifier = Identifier::minecraft("worldgen/biome").unwrap();
        let plains_biome_namespace = Identifier::minecraft("plains").unwrap();

        let plains_biome_id = registry
            .get_numeric_id(&biome_registry_identifier, &plains_biome_namespace)
            .unwrap() as u8;
        Self::new(
            [Air.get_state_id(); AMOUNT_OF_BLOCKS_IN_CHUNK_SECTION],
            [plains_biome_id; AMOUNT_OF_BIOMES_IN_CHUNK_SECTION],
        )
    }
}
