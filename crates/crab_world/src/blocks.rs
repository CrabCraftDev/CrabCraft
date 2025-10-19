mod block;
pub mod block_data;
pub mod block_information;
pub use block::*;
use crab_world::error::BlockError;

include!(concat!(env!("OUT_DIR"), "/blocks.rs"));
