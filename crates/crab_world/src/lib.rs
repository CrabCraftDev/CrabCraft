extern crate self as crab_world;

use crate::blocks_registry::BlocksRegistry;
use std::sync::OnceLock;

pub use block::*;

mod block;
pub mod block_data;
pub mod blocks;
pub mod blocks_registry;
pub mod error;
pub mod paletted_container;

pub static BLOCKS: OnceLock<BlocksRegistry> = OnceLock::new();
