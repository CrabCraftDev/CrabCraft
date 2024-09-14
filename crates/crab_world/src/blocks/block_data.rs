use std::collections::HashMap;

use crab_macros::BlockData;

use crate::error::WorldError;

/// The `BlockData` trait provides a way to create typed access to block properties/data.
///
/// # Example
///
/// ```rust
/// use crab_world::blocks::{BlockType, Block, BLOCKS, init_blocks, GrassBlock};
/// use crab_world::blocks::block_data::Snowy;
/// # BLOCKS.get_or_init(init_blocks);
/// let block = GrassBlock::default_state_id().unwrap();
/// let is_snowy = block.property::<Snowy>().unwrap();
/// ```
pub trait BlockData {
    type Value;

    fn get(self) -> Self::Value;

    fn from_properties(properties: HashMap<&'static str, String>) -> Result<Self, WorldError>
    where
        Self: Sized;

    fn apply(property: &Self::Value, properties: &mut HashMap<&'static str, String>) -> Result<(), WorldError>
    where
        Self: Sized;
}

#[derive(Debug, Clone, PartialEq, Eq, BlockData)]
pub struct Snowy {
    pub snowy: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, BlockData)]
pub struct Waterlogged {
    pub waterlogged: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, BlockData)]
pub struct RedstoneWireData {
    pub power: u16,
    pub north: bool,
    pub east: bool,
    pub south: bool,
    pub west: bool,
}
