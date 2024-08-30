use crate::block::Block;
use crate::error::WorldError;
use crab_protocol::types::namespaced_key::NamespacedKey;
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Clone)]
pub struct BlockInformation {
    pub from_properties: fn(hashmap: &HashMap<String, String>) -> Result<Block, WorldError>,
    pub properties: fn(local_state: u16) -> Result<HashMap<String, u16>, WorldError>,
    pub starting_state_id: u16,
    pub default_state_id: u16,
}

#[derive(Debug, Default, Clone)]
pub struct BlockRegistry {
    pub last_state_id: u16,
    pub(crate) block_information_by_key: HashMap<NamespacedKey, BlockInformation>,
    pub(crate) by_state_id: BTreeMap<u16, NamespacedKey>,
}

impl BlockRegistry {
    pub fn add_block(
        &mut self,
        states_amount: u16,
        namespaced_key: &NamespacedKey,
        mut information: BlockInformation,
    ) -> Result<(), WorldError> {
        if self.block_information_by_key.contains_key(namespaced_key) {
            return Err(WorldError::DuplicateBlock(namespaced_key.clone()));
        }

        information.starting_state_id = self.last_state_id;
        information.default_state_id += information.starting_state_id;
        self.block_information_by_key
            .insert(namespaced_key.clone(), information);
        self.by_state_id.insert(self.last_state_id, namespaced_key.clone());
        self.last_state_id += states_amount;

        Ok(())
    }

    pub fn namespaced_key_by_state_id(&self, key: u16) -> Result<&NamespacedKey, WorldError> {
        if key > self.last_state_id {
            return Err(WorldError::BlockIdDoesNotExist(key));
        }
        let block_id = self.by_state_id.range(..=key).next_back().map(|(_, v)| v);
        block_id.ok_or(WorldError::BlockIdDoesNotExist(key))
    }

    pub fn block_information_by_namespaced_key(
        &self,
        namespaced_key: &NamespacedKey,
    ) -> Result<&BlockInformation, WorldError> {
        self.block_information_by_key
            .get(namespaced_key)
            .ok_or(WorldError::BlockDoesNotExist(namespaced_key.clone()))
    }
}
