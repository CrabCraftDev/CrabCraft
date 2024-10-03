use crab_world::blocks::block_registry::BlockRegistry;
use crab_world::error::WorldError;
use std::sync::OnceLock;

mod block;
pub mod block_data;
pub mod block_registry;
pub use block::*;

include!(concat!(env!("OUT_DIR"), "/block"));

pub static BLOCKS: OnceLock<BlockRegistry> = OnceLock::new();

fn get_block_registry() -> Result<&'static BlockRegistry, WorldError> {
    BLOCKS.get().ok_or(WorldError::BlocksNotInitialized)
}
