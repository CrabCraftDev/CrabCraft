use crate::blocks::block_registry::BlockRegistry;
use std::sync::OnceLock;

mod block;
pub mod block_data;
pub mod block_registry;
pub use block::*;
use crab_world::error::BlockError;

include!(concat!(env!("OUT_DIR"), "/blocks.rs"));

pub static BLOCKS: OnceLock<BlockRegistry> = OnceLock::new();

fn get_block_registry() -> Result<&'static BlockRegistry, BlockError> {
    BLOCKS.get().ok_or(BlockError::BlocksNotInitialized)
}
