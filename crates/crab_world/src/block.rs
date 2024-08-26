use std::collections::HashMap;

use derive_more::{From, Into};

use crab_protocol::types::namespaced_key::NamespacedKey;

use crate::block_data::BlockData;
use crate::blocks_registry::BlockInformation;
use crate::error::WorldError;
use crate::BLOCKS;

/// Represents a block
#[derive(Debug, Clone, Copy, Into, From)]
#[repr(transparent)]
pub struct Block(pub u16);

impl Block {
    pub fn get_namespaced_key(&self) -> Result<&NamespacedKey, WorldError> {
        BLOCKS
            .get()
            .ok_or(WorldError::BlocksNotInitialized)?
            .get_block_from_id(self.0)
    }

    pub fn get_properties(&self) -> Result<HashMap<String, u16>, WorldError> {
        let namespaced_key = self.get_namespaced_key()?;
        let blocks = BLOCKS.get().ok_or(WorldError::BlocksNotInitialized)?;
        let block_information = blocks.get_by_id(namespaced_key)?;
        let local_state = self.0 - block_information.starting_id;
        let properties = (block_information.get_properties)(local_state)?;

        Ok(properties)
    }

    pub fn get_from_properties(
        namespaced_key: &NamespacedKey,
        properties: &HashMap<String, String>,
    ) -> Result<Block, WorldError> {
        let blocks = BLOCKS.get().ok_or(WorldError::BlocksNotInitialized)?;
        let block_information = blocks.get_by_id(namespaced_key)?;
        (block_information.from_properties)(properties)
    }

    pub fn is_same_as_namespaced_key(&self, other: Result<&NamespacedKey, WorldError>) -> bool {
        let self_namespaced_key = self.get_namespaced_key();

        if let (Ok(self_namespaced_key), Ok(other_namespaced_key)) = (self_namespaced_key, other) {
            return self_namespaced_key == other_namespaced_key;
        }

        false
    }

    pub fn is_same_as(&self, other: &Block) -> bool {
        self.is_same_as_namespaced_key(other.get_namespaced_key())
    }

    pub fn get_property<P: BlockData>(&self) -> Result<P, WorldError> {
        P::from_properties(self.get_properties()?)
    }
}

/// Represents generated block (for example StoneBlock)
/// With methods for parsing its protocol ID
pub trait BlockType {
    /// Returns unique NamespacedKey of block
    /// For example `minecraft:stone`
    fn get_namespaced_key() -> NamespacedKey;

    fn get_properties(local_state: u16) -> Result<HashMap<String, u16>, WorldError>;
    fn from_properties(hashmap: &HashMap<String, String>) -> Result<Block, WorldError>;

    fn is_same_type_as(block: &Block) -> bool {
        block.is_same_as_namespaced_key(Ok(&Self::get_namespaced_key()))
    }

    fn get_information() -> Result<&'static BlockInformation, WorldError> {
        let blocks = BLOCKS.get().ok_or(WorldError::BlocksNotInitialized)?;
        blocks.get_by_id(&Self::get_namespaced_key())
    }

    fn get_default_id() -> Result<Block, WorldError> {
        Ok(Block(Self::get_information()?.default_id))
    }

    fn get_starting_id() -> Result<Block, WorldError> {
        Ok(Block(Self::get_information()?.starting_id))
    }
}
