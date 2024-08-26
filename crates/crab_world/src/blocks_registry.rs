use crate::block::Block;
use crate::error::WorldError;
use crab_protocol::types::identifier::Identifier;
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
    pub(crate) blocks_information: HashMap<Identifier, BlockInformation>,
    pub(crate) id_to_block: BTreeMap<u16, Identifier>,
}

impl BlocksRegistry {
    pub fn add_block(
        &mut self,
        states_amount: u16,
        identifier: &Identifier,
        mut information: BlockInformation,
    ) -> Result<(), WorldError> {
        if self.blocks_information.contains_key(identifier) {
            return Err(WorldError::DuplicateBlock(identifier.clone()));
        }

        information.starting_id = self.current_id;
        information.default_id += information.starting_id;
        self.blocks_information.insert(identifier.clone(), information);
        self.id_to_block.insert(self.current_id, identifier.clone());
        self.current_id += states_amount;

        Ok(())
    }

    pub fn get_block_from_id(&self, key: u16) -> Result<&Identifier, WorldError> {
        if key > self.current_id {
            return Err(WorldError::BlockIdDoesNotExist(key));
        }
        let block_id = self.id_to_block.range(..=key).next_back().map(|(_, v)| v);
        block_id.ok_or(WorldError::BlockIdDoesNotExist(key))
    }

    pub fn get_by_id(&self, identifier: &Identifier) -> Result<&BlockInformation, WorldError> {
        self.blocks_information
            .get(identifier)
            .ok_or(WorldError::BlockDoesNotExist(identifier.clone()))
    }
}
