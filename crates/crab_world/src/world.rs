use crate::error::WorldError;
use crab_registry::REGISTRY;
use crab_text::utils::namespaced_key::NamespacedKey;
use crab_world::anvil::region::file::RegionFile;
use crab_world::chunks::chunk::Chunk;
use crab_world::chunks::section::CHUNK_SIZE_SHIFT;
use dashmap::DashMap;
use dashmap::mapref::one::{Ref, RefMut};
use nohash_hasher::BuildNoHashHasher;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use tracing::warn;

mod blocks;
mod region;

const REGION_SIZE_IN_CHUNKS: i32 = 32;

type ChunkCoordinates = i64;

#[derive(Debug)]
pub struct World {
    pub identifier: NamespacedKey,
    pub path: PathBuf,
    pub min_y: i16,
    pub chunks: Arc<DashMap<ChunkCoordinates, Chunk, BuildNoHashHasher<ChunkCoordinates>>>,
    pub regions: DashMap<(i32, i32), RegionFile>,
}

impl World {
    pub fn save(&self) -> Result<(), WorldError> {
        // FIXME: Now we are saving all chunks, even those that haven't changed.
        for chunk in self.chunks.iter() {
            let (x, z) = Self::unpack_key(*chunk.key());
            self.save_chunk(x, z, chunk.value())?;
        }
        Ok(())
    }

    pub fn get_chunk(&self, x: i32, z: i32) -> Option<Ref<'_, ChunkCoordinates, Chunk>> {
        self.chunks.get(&Self::get_key(x, z))
    }

    pub fn get_chunk_or_load(&self, x: i32, z: i32) -> Result<Ref<'_, ChunkCoordinates, Chunk>, WorldError> {
        let key = Self::get_key(x, z);
        if let Some(chunk) = self.chunks.get(&key) {
            Ok(chunk)
        } else {
            self.load_chunk(x, z)?;
            Ok(self.chunks.get(&key).expect("Unreachable: Chunk was just loaded"))
        }
    }

    pub fn get_chunk_mut(&self, x: i32, z: i32) -> Option<RefMut<'_, ChunkCoordinates, Chunk>> {
        self.chunks.get_mut(&Self::get_key(x, z))
    }

    pub fn get_chunk_mut_or_load(&self, x: i32, z: i32) -> Result<RefMut<'_, ChunkCoordinates, Chunk>, WorldError> {
        let key = Self::get_key(x, z);
        if let Some(chunk) = self.chunks.get_mut(&key) {
            Ok(chunk)
        } else {
            self.load_chunk(x, z)?;
            Ok(self.chunks.get_mut(&key).expect("Unreachable: Chunk was just loaded"))
        }
    }

    pub fn load_chunk(&self, x: i32, z: i32) -> Result<(), WorldError> {
        let key = Self::get_key(x, z);

        match self.load_chunk_from_anvil(x, z) {
            Ok(chunk) => {
                self.chunks.insert(key, chunk);
                Ok(())
            }
            Err(WorldError::MissingWorldFile { .. }) => {
                warn!("World generator is not implemented in CrabCraft!");
                warn!("Ensure the required pre-generated world files exist in: worlds/overworld/region/");

                self.chunks.insert(key, Chunk::default());
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    fn load_chunk_from_anvil(&self, x: i32, z: i32) -> Result<Chunk, WorldError> {
        let mut region = self.get_or_load_region_file(x, z)?;

        let (local_chunk_x, local_chunk_z) = Self::chunk_coordinates_inside_region(x, z);
        let chunk = region.read_chunk(
            local_chunk_x,
            local_chunk_z,
            Self::get_biome,
            REGISTRY.get().expect("TODO: Throw CrabRegistry error"),
        )?;

        Ok(chunk)
    }

    pub fn save_chunk(&self, x: i32, z: i32, chunk: &Chunk) -> Result<(), WorldError> {
        let mut region = self.get_or_load_region_file(x, z)?;

        let (local_chunk_x, local_chunk_z) = Self::chunk_coordinates_inside_region(x, z);
        region.write_chunk(local_chunk_x, local_chunk_z, chunk, REGISTRY.get().unwrap())?;

        Ok(())
    }

    pub fn unload_chunk(&mut self, x: i32, z: i32) {
        self.chunks.remove(&Self::get_key(x, z));
    }

    fn get_biome(biome: &str) -> u64 {
        let registry = REGISTRY.get().unwrap();
        let biome_registry_namespaced_key = NamespacedKey::minecraft("worldgen/biome").unwrap();

        registry
            .get_numeric_id(&biome_registry_namespaced_key, &NamespacedKey::from_str(biome).unwrap())
            .expect("Invalid biome id") as u64
    }

    // FIXME: Create struct ChunkCoordinates and move these methods there (#255)
    pub fn get_key(region_x: i32, region_z: i32) -> ChunkCoordinates {
        ((region_x as ChunkCoordinates) << i32::BITS) | region_z as ChunkCoordinates
    }

    pub fn unpack_key(key: ChunkCoordinates) -> (i32, i32) {
        let region_x = (key >> i32::BITS) as i32;
        let region_z = key as i32;
        (region_x, region_z)
    }

    fn chunk_coordinates(block_x: i32, block_z: i32) -> (i32, i32) {
        (block_x >> CHUNK_SIZE_SHIFT, block_z >> CHUNK_SIZE_SHIFT)
    }

    fn chunk_coordinates_inside_region(x: i32, z: i32) -> (u8, u8) {
        ((x % REGION_SIZE_IN_CHUNKS) as u8, (z % REGION_SIZE_IN_CHUNKS) as u8)
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
}
