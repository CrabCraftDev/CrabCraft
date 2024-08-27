use crate::utils::errors::Error;
use crate::REGISTRY;
use bytes::{BufMut, Bytes, BytesMut};
use crab_protocol::types::namespaced_key::NamespacedKey;
use crab_world::blocks::Air;
use crab_world::paletted_container::{
    get_biomes_container, get_blocks_container, AMOUNT_OF_BIOMES_IN_CHUNK_SECTION, AMOUNT_OF_BLOCKS_IN_CHUNK_SECTION,
};
use crab_world::{Block, BlockType, BLOCKS};

/// Chunk section is a 16×16×16 collection of blocks.
/// It stores blocks and biomes in a [paletted](crab_world::paletted_container::PalettedContainer) format.
#[derive(Debug, Clone)]
pub struct ChunkSection {
    pub non_air_block_amount: u16,
    pub blocks: Box<[Block; AMOUNT_OF_BLOCKS_IN_CHUNK_SECTION]>,
    pub biomes: Box<[u8; AMOUNT_OF_BIOMES_IN_CHUNK_SECTION]>,
    pub cached_bytes: Option<Bytes>,
}

impl ChunkSection {
    pub fn new(
        blocks: [Block; AMOUNT_OF_BLOCKS_IN_CHUNK_SECTION],
        biomes: [u8; AMOUNT_OF_BIOMES_IN_CHUNK_SECTION],
    ) -> Self {
        Self {
            non_air_block_amount: blocks.iter().filter(|&v| Air::is_same_type_as(v)).count() as u16,
            blocks: Box::new(blocks),
            biomes: Box::new(biomes),
            cached_bytes: None,
        }
    }

    pub fn set_block(&mut self, x: u8, y: u8, z: u8, block: Block) {
        self.cached_bytes = None;

        let index = self.calculate_index(x, y, z);
        let previous_block = self.blocks[index];

        if Air::is_same_type_as(&previous_block) && !Air::is_same_type_as(&block) {
            self.non_air_block_amount += 1;
        } else if !Air::is_same_type_as(&previous_block) && Air::is_same_type_as(&block) {
            self.non_air_block_amount -= 1;
        }

        self.blocks[index] = block
    }

    #[allow(dead_code)]
    pub fn get_block(&self, x: u8, y: u8, z: u8) -> Block {
        self.blocks[self.calculate_index(x, y, z)]
    }

    pub fn serialize_to_mc_format(&mut self, chunk_section: &mut BytesMut) -> Result<(), Error> {
        if let Some(cached_bytes) = &self.cached_bytes {
            chunk_section.put_slice(cached_bytes);
            return Ok(());
        }

        let mut bytes = BytesMut::new();
        bytes.put_i16(self.non_air_block_amount as i16);
        get_blocks_container(BLOCKS.get().unwrap().get_max_used_id() as usize)
            .serialize_palette(self.blocks.iter().map(|&v| v.0 as i32).collect(), &mut bytes)?;

        let registry = &REGISTRY.read().unwrap();
        let biome_registry_namespaced_key = NamespacedKey::minecraft("worldgen/biome")?;
        let biomes_amount = registry.get_amount(&biome_registry_namespaced_key).unwrap();

        get_biomes_container(biomes_amount)
            .serialize_palette(self.biomes.iter().map(|&v| v as i32).collect(), &mut bytes)?;

        let bytes = bytes.freeze();

        chunk_section.put(bytes.clone());
        self.cached_bytes = Some(bytes);

        Ok(())
    }

    fn calculate_index(&self, x: u8, y: u8, z: u8) -> usize {
        (y as usize * 16 * 16) + (z as usize * 16) + x as usize
    }
}

impl Default for ChunkSection {
    fn default() -> Self {
        let registry = &REGISTRY.read().unwrap();
        let biome_registry_namespaced_key = NamespacedKey::minecraft("worldgen/biome").unwrap();
        let plains_biome_namespace = NamespacedKey::minecraft("plains").unwrap();

        let plains_biome_id = registry
            .get_numeric_id(&biome_registry_namespaced_key, &plains_biome_namespace)
            .unwrap() as u8;
        Self::new(
            [Air::get_default_id().unwrap(); AMOUNT_OF_BLOCKS_IN_CHUNK_SECTION],
            [plains_biome_id; AMOUNT_OF_BIOMES_IN_CHUNK_SECTION],
        )
    }
}
