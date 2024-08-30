extern crate self as crab_world;

use crate::block_registry::BlockRegistry;
use std::sync::OnceLock;

use crate::error::WorldError;
pub use block::*;

mod block;
pub mod block_data;
pub mod block_registry;
pub mod blocks;
pub mod chunks;
pub mod error;
pub mod paletted_container;

pub static BLOCKS: OnceLock<BlockRegistry> = OnceLock::new();

fn get_block_registry() -> Result<&'static BlockRegistry, WorldError> {
    BLOCKS.get().ok_or(WorldError::BlocksNotInitialized)
}
