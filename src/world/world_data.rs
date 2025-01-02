use crate::utils::errors::Error;
use crate::utils::files::FILES;
use crate::world::OVERWORLD_IDENTIFIER;
use bytes::{Bytes, BytesMut};
use crab_protocol::types::namespaced_key::NamespacedKey;
use crab_registry::REGISTRY;
use crab_world::anvil::region::Region;
use crab_world::blocks::Block;
use crab_world::chunks::chunk::Chunk;
use crab_world::chunks::section::CHUNK_SIZE_SHIFT;
use crab_world::error::AnvilError::ChunkNotGenerated;
use dashmap::mapref::one::{Ref, RefMut};
use dashmap::DashMap;
use nohash_hasher::BuildNoHashHasher;
use std::fs;
use std::str::FromStr;
use std::sync::Arc;
use tracing::warn;

const REGION_SIZE_IN_CHUNKS: i32 = 32;

type ChunkCoordinates = i64;

#[derive(Debug, Default)]
pub struct World {
    pub min_y: i16,
    pub chunks: Arc<DashMap<ChunkCoordinates, Chunk, BuildNoHashHasher<ChunkCoordinates>>>,
}

impl World {
    pub fn serialize_chunk(&self, bytes: &mut BytesMut, x: i32, z: i32) -> Result<(), Error> {
        let chunk = self.get_chunk_or_load(x, z)?;
        for section in &chunk.sections {
            section.serialize_to_mc_format(bytes)?;
        }
        Ok(())
    }

    pub fn get_block(&self, x: i32, y: i16, z: i32) -> Result<Block, Error> {
        let (chunk_x, chunk_z) = Self::chunk_coordinates(x, z);
        let Some(chunk) = self.get_chunk(chunk_x, chunk_z) else {
            return Err(Error::AccessUnloadedChunk { x, z });
        };

        chunk
            .get_block((x % 16) as u8, (y - self.min_y) as u16, (z % 16) as u8)
            .ok_or(Error::InvalidPositionInChunk)
    }

    pub fn set_block(&self, x: i32, y: i16, z: i32, block: Block) -> Result<(), Error> {
        let (chunk_x, chunk_z) = Self::chunk_coordinates(x, z);
        let Some(mut chunk) = self.get_chunk_mut(chunk_x, chunk_z) else {
            return Err(Error::AccessUnloadedChunk { x, z });
        };

        Ok(chunk.set_block((x % 16) as u8, (y - self.min_y) as u16, (z % 16) as u8, block)?)
    }

    pub fn get_chunk(&self, x: i32, z: i32) -> Option<Ref<'_, ChunkCoordinates, Chunk>> {
        self.chunks.get(&Self::get_key(x, z))
    }

    pub fn get_chunk_or_load(&self, x: i32, z: i32) -> Result<Ref<'_, ChunkCoordinates, Chunk>, Error> {
        let key = Self::get_key(x, z);
        if let Some(chunk) = self.chunks.get(&key) {
            Ok(chunk)
        } else {
            self.load_chunk(x, z)?;
            self.chunks.get(&key).ok_or(Error::FailedToLoadChunk { x, z })
        }
    }

    pub fn get_chunk_mut(&self, x: i32, z: i32) -> Option<RefMut<'_, ChunkCoordinates, Chunk>> {
        self.chunks.get_mut(&Self::get_key(x, z))
    }

    pub fn get_chunk_mut_or_load(&self, x: i32, z: i32) -> Result<RefMut<'_, ChunkCoordinates, Chunk>, Error> {
        let key = Self::get_key(x, z);
        if let Some(chunk) = self.chunks.get_mut(&key) {
            Ok(chunk)
        } else {
            self.load_chunk(x, z)?;
            self.chunks.get_mut(&key).ok_or(Error::FailedToLoadChunk { x, z })
        }
    }

    pub fn load_chunk(&self, x: i32, z: i32) -> Result<(), Error> {
        let key = Self::get_key(x, z);

        match self.load_chunk_from_anvil(x, z) {
            Ok(chunk) => {
                self.chunks.insert(key, chunk);
                Ok(())
            }
            Err(Error::Anvil(ChunkNotGenerated { .. })) => {
                warn!("World generator is not implemented in CrabCraft!");
                warn!("Ensure the required pre-generated world files exist in: worlds/overworld/region/");

                self.chunks.insert(key, Chunk::default());
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    fn load_chunk_from_anvil(&self, x: i32, z: i32) -> Result<Chunk, Error> {
        let region_folder_path = FILES.world_region_path(&OVERWORLD_IDENTIFIER)?;
        let region_file_name = Self::get_region_name(x, z);

        let bytes = fs::read(region_folder_path.join(&region_file_name))
            .map_err(|_| Error::MissingWorldFile(region_file_name.clone()))?;

        let region = Region::from(Bytes::from(bytes));
        let (local_chunk_x, local_chunk_z) = Self::chunk_coordinates_inside_region(x, z);
        let chunk = region.read_chunk(local_chunk_x, local_chunk_z, Self::get_biome)?;

        Ok(chunk)
    }

    pub fn unload_chunk(&mut self, x: i32, z: i32) {
        self.chunks.remove(&Self::get_key(x, z));
    }

    fn get_biome(biome: &str) -> u64 {
        let registry = REGISTRY.get().unwrap();
        let biome_registry_namespaced_key = NamespacedKey::minecraft("worldgen/biome").unwrap();

        registry
            .get_numeric_id(&biome_registry_namespaced_key, &NamespacedKey::from_str(biome).unwrap())
            .unwrap() as u64
    }

    pub fn get_key(region_x: i32, region_z: i32) -> ChunkCoordinates {
        ((region_x as ChunkCoordinates) << i32::BITS) | region_z as ChunkCoordinates
    }

    fn chunk_coordinates(block_x: i32, block_z: i32) -> (i32, i32) {
        (block_x >> CHUNK_SIZE_SHIFT, block_z >> CHUNK_SIZE_SHIFT)
    }

    fn chunk_coordinates_inside_region(x: i32, z: i32) -> (u8, u8) {
        ((x % REGION_SIZE_IN_CHUNKS) as u8, (z % REGION_SIZE_IN_CHUNKS) as u8)
    }

    fn get_region_name(chunk_x: i32, chunk_z: i32) -> String {
        // Floor of dividing the chunk coordinates by 32 (REGION_SIZE_IN_CHUNKS)
        format!("r.{}.{}.mca", chunk_x >> 5, chunk_z >> 5)
    }
}

#[cfg(test)]
mod tests {
    use crate::world::World;

    #[test]
    fn test_chunk_coordinates() {
        let (chunk_x, chunk_z) = World::chunk_coordinates(32, -5);
        assert_eq!(chunk_x, 2);
        assert_eq!(chunk_z, -1);
    }

    #[test]
    fn test_get_region_name() {
        let region = World::get_region_name(1500, -600);

        assert_eq!(region, "r.46.-19.mca");
    }
}
