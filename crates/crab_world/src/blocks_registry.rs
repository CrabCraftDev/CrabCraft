use crate::block::Block;
use crate::error::WorldError;
use crab_protocol::types::namespaced_key::NamespacedKey;
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Clone)]
pub struct BlockInformation {
    pub from_properties: fn(hashmap: &HashMap<String, String>) -> Result<Block, WorldError>,
    pub get_properties: fn(local_state: u16) -> Result<HashMap<String, u16>, WorldError>,
    pub starting_id: u16,
    pub default_id: u16,
}

#[derive(Debug, Default, Clone)]
pub struct BlocksRegistry {
    pub current_id: u16,
    pub(crate) blocks_information: HashMap<NamespacedKey, BlockInformation>,
    pub(crate) id_to_block: BTreeMap<u16, NamespacedKey>,
}

impl BlocksRegistry {
    pub fn add_block(
        &mut self,
        states_amount: u16,
        namespaced_key: &NamespacedKey,
        mut information: BlockInformation,
    ) -> Result<(), WorldError> {
        if self.blocks_information.contains_key(namespaced_key) {
            return Err(WorldError::DuplicateBlock(namespaced_key.clone()));
        }

        information.starting_id = self.current_id;
        information.default_id += information.starting_id;
        self.blocks_information.insert(namespaced_key.clone(), information);
        self.id_to_block.insert(self.current_id, namespaced_key.clone());
        self.current_id += states_amount;

        Ok(())
    }

    pub fn get_block_from_id(&self, key: u16) -> Result<&NamespacedKey, WorldError> {
        if key > self.get_max_used_id() {
            return Err(WorldError::BlockIdDoesNotExist(key));
        }
        let block_id = self.id_to_block.range(..=key).next_back().map(|(_, v)| v);
        block_id.ok_or(WorldError::BlockIdDoesNotExist(key))
    }

    pub fn get_by_id(&self, namespaced_key: &NamespacedKey) -> Result<&BlockInformation, WorldError> {
        self.blocks_information
            .get(namespaced_key)
            .ok_or(WorldError::BlockDoesNotExist(namespaced_key.clone()))
    }

    pub fn get_max_used_id(&self) -> u16 {
        self.current_id
    }
}
