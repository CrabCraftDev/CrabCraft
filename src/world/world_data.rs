use crate::utils::errors::Error;
use crate::utils::errors::Error::FailedToLoadRegistry;
use crate::utils::files::FILES;
use crate::world::OVERWORLD_IDENTIFIER;
use bytes::BytesMut;
use crab_registry::REGISTRY;
use crab_text::utils::namespaced_key::NamespacedKey;
use crab_world::anvil::region::file::RegionFile;
use crab_world::blocks::Block;
use crab_world::chunks::chunk::Chunk;
use crab_world::chunks::section::CHUNK_SIZE_SHIFT;
use crab_world::error::AnvilError::ChunkDoesNotExist;
use dashmap::DashMap;
use dashmap::mapref::one::{Ref, RefMut};
use nohash_hasher::BuildNoHashHasher;
use std::fs::OpenOptions;
use std::str::FromStr;
use std::sync::Arc;
use tracing::warn;

const REGION_SIZE_IN_CHUNKS: i32 = 32;

type ChunkCoordinates = i64;

#[derive(Debug, Default)]
pub struct World {
    pub min_y: i16,
    pub chunks: Arc<DashMap<ChunkCoordinates, Chunk, BuildNoHashHasher<ChunkCoordinates>>>,
    pub regions: DashMap<(i32, i32), RegionFile>,
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
            Err(Error::Anvil(ChunkDoesNotExist { .. })) | Err(Error::MissingWorldFile { .. }) => {
                warn!("World generator is not implemented in CrabCraft!");
                warn!("Ensure the required pre-generated world files exist in: worlds/overworld/region/");

                self.chunks.insert(key, Chunk::default());
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    fn load_chunk_from_anvil(&self, x: i32, z: i32) -> Result<Chunk, Error> {
        let mut region = self.get_or_load_region_file(x, z)?;

        let (local_chunk_x, local_chunk_z) = Self::chunk_coordinates_inside_region(x, z);
        let chunk = region.read_chunk(
            local_chunk_x,
            local_chunk_z,
            Self::get_biome,
            REGISTRY.get().ok_or(FailedToLoadRegistry)?,
        )?;

        Ok(chunk)
    }

    fn get_or_load_region_file(&self, x: i32, z: i32) -> Result<RefMut<'_, (i32, i32), RegionFile>, Error> {
        let region_folder_path = FILES.world_region_path(&OVERWORLD_IDENTIFIER)?;
        let region_file_name = Self::get_region_name(x, z);
        let region_coordinates = Self::region_coordinates(x, z);

        if !self.regions.contains_key(&region_coordinates) {
            let file = OpenOptions::new()
                .read(true)
                .write(true)
                .open(region_folder_path.join(&region_file_name))
                .map_err(|_| Error::MissingWorldFile(region_file_name.clone()))?;

            let region_file = RegionFile::load(file)?;
            self.regions.insert(region_coordinates, region_file);
        }

        Ok(self.regions.get_mut(&region_coordinates).unwrap())
    }

    pub fn save_chunk(&self, x: i32, z: i32, chunk: &Chunk) -> Result<(), Error> {
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

    // todo: move those methods to impl ChunkCoordinates
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

    fn region_coordinates(x: i32, z: i32) -> (i32, i32) {
        // Floor of dividing the chunk coordinates by 32 (REGION_SIZE_IN_CHUNKS)
        (x >> 5, z >> 5)
    }

    fn get_region_name(chunk_x: i32, chunk_z: i32) -> String {
        let (region_x, region_z) = Self::region_coordinates(chunk_x, chunk_z);
        format!("r.{}.{}.mca", region_x, region_z)
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
