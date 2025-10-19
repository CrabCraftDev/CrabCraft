use crate::blocks::Block;
use crate::error::BlockError;
use crab_registry::registry_key::RegistryKey;
use std::collections::HashMap;

// todo: maybe new arc
// pub fn get_block_registry() -> &'static Registry<BlockInformation> {
//     let registry = REGISTRY.load();
//     registry.get_registry(BlockInformation::REGISTRY).unwrap()
// }

#[derive(Debug, Clone)]
pub struct BlockInformation {
    pub from_properties: fn(hashmap: &HashMap<&str, String>) -> Result<Block, BlockError>,
    pub properties: fn(local_state: u16) -> Result<HashMap<&'static str, String>, BlockError>,
    pub starting_state_id: u16,
    pub default_state_id: u16,
}

impl BlockInformation {
    pub const REGISTRY: RegistryKey<Self> = RegistryKey::new("crabcraft:block_state");
}
