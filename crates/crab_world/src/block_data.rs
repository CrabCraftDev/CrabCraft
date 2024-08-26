use std::collections::HashMap;

use crab_macros::BlockData;

use crate::error::WorldError;

/// The `BlockData` trait provides a way to create typed access to block properties/data.
///
/// # Example
///
/// ```ignore
/// let block = GrassBlock::get_default_id().unwrap();
/// let is_snowy = block.get_property::<Snowy>().unwrap().snowy;
/// ```
pub trait BlockData {
    fn from_properties(properties: HashMap<String, u16>) -> Result<Self, WorldError>
    where
        Self: Sized;
}

#[derive(Debug, Clone, PartialEq, Eq, BlockData)]
pub struct Snowy {
    pub snowy: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, BlockData)]
pub struct Waterlogged {
    pub snowy: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, BlockData)]
pub struct RedstoneWire {
    power: u16,
    north: bool,
    east: bool,
    south: bool,
    west: bool,
}
