use std::collections::HashMap;

use crab_macros::BlockData;

use crate::error::WorldError;

/// The `BlockData` trait provides a way to create typed access to block properties/data.
///
/// # Example
///
/// ```ignore
/// let block = GrassBlock::default_state_id().unwrap();
/// let is_snowy = block.property::<Snowy>().unwrap().snowy;
/// ```
pub trait BlockData {
    fn from_properties(properties: HashMap<&'static str, u16>) -> Result<Self, WorldError>
    where
        Self: Sized;

    fn apply(&self, properties: &mut HashMap<&'static str, u16>) -> Result<(), WorldError>
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
