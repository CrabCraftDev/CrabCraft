use crate::world::World;
use crab_world::blocks::Block;
use crab_world::error::WorldError;

impl World {
    pub fn get_block(&self, x: i32, y: i16, z: i32) -> Result<Block, WorldError> {
        let (chunk_x, chunk_z) = Self::chunk_coordinates(x, z);
        let Some(chunk) = self.get_chunk(chunk_x, chunk_z) else {
            return Err(WorldError::AccessUnloadedChunk { x, z });
        };

        chunk
            .get_block((x % 16) as u8, (y - self.min_y) as u16, (z % 16) as u8)
            .ok_or(WorldError::InvalidPositionInChunk)
    }

    pub fn set_block(&self, x: i32, y: i16, z: i32, block: Block) -> Result<(), WorldError> {
        let (chunk_x, chunk_z) = Self::chunk_coordinates(x, z);
        let Some(mut chunk) = self.get_chunk_mut(chunk_x, chunk_z) else {
            return Err(WorldError::AccessUnloadedChunk { x, z });
        };

        chunk.set_block((x % 16) as u8, (y - self.min_y) as u16, (z % 16) as u8, block)
    }
}
