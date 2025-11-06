use crate::blocks::{Block, init_blocks};
use crate::error::BlockError;
use crab_registry::registry_key::RegistryKey;
use crab_registry::{REGISTRY_POPULATORS, RootRegistry};
use linkme::distributed_slice;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct BlockInformation {
    pub from_properties: fn(hashmap: &HashMap<&str, String>) -> Result<Block, BlockError>,
    pub properties: fn(local_state: u16) -> Result<HashMap<&'static str, String>, BlockError>,
    pub starting_state_id: u16,
    pub default_state_id: u16,
}

#[distributed_slice(REGISTRY_POPULATORS)]
static BLOCKS_REGISTRY_POPULATOR: fn(&mut RootRegistry) = init_blocks;

impl BlockInformation {
    pub const REGISTRY: RegistryKey<Self> = RegistryKey::new("crabcraft:block_state");
}
