use std::collections::HashMap;

use derive_more::{From, Into};

use crab_protocol::types::namespaced_key::NamespacedKey;

use crate::block_data::BlockData;
use crate::block_registry::BlockInformation;
use crate::error::WorldError;
use crate::{get_block_registry, BLOCKS};

/// Represents a block used in our codebase
/// u16 is block state id (mostly used in protocol), which is unique id for each block state
#[derive(Debug, Clone, Copy, Into, From)]
#[repr(transparent)]
pub struct Block(pub u16);

impl Block {
    pub fn namespaced_key(&self) -> Result<&NamespacedKey, WorldError> {
        BLOCKS
            .get()
            .ok_or(WorldError::BlocksNotInitialized)?
            .namespaced_key_by_state_id(self.0)
    }

    pub fn properties(&self) -> Result<HashMap<String, u16>, WorldError> {
        let namespaced_key = self.namespaced_key()?;
        let block_information = get_block_registry()?.block_information_by_namespaced_key(namespaced_key)?;
        let local_state = self.0 - block_information.starting_state_id;
        let properties = (block_information.properties)(local_state)?;

        Ok(properties)
    }

    pub fn from_properties(
        namespaced_key: &NamespacedKey,
        properties: &HashMap<String, String>,
    ) -> Result<Block, WorldError> {
        let block_information = get_block_registry()?.block_information_by_namespaced_key(namespaced_key)?;
        (block_information.from_properties)(properties)
    }

    pub fn is_same_as(&self, other: &Block) -> bool {
        matches!((self.namespaced_key(), other.namespaced_key()), (Ok(own_key), Ok(other_key)) if own_key == other_key)
    }

    pub fn get_property<P: BlockData>(&self) -> Result<P, WorldError> {
        P::from_properties(self.properties()?)
    }
}

/// Represents generated block (for example StoneBlock)
/// With methods for parsing its protocol ID
pub trait BlockType {
    /// Returns unique NamespacedKey of block
    /// For example `minecraft:stone`
    fn namespaced_key() -> NamespacedKey;

    fn properties(local_state: u16) -> Result<HashMap<String, u16>, WorldError>;
    fn from_properties(hashmap: &HashMap<String, String>) -> Result<Block, WorldError>;

    fn is_same_type_as(block: &Block) -> bool {
        matches!(block.namespaced_key(), Ok(block_key) if *block_key == Self::namespaced_key())
    }

    fn information() -> Result<&'static BlockInformation, WorldError> {
        let blocks = get_block_registry()?;
        blocks.block_information_by_namespaced_key(&Self::namespaced_key())
    }

    fn default_state_id() -> Result<Block, WorldError> {
        Ok(Block(Self::information()?.default_state_id))
    }

    fn starting_state_id() -> Result<Block, WorldError> {
        Ok(Block(Self::information()?.starting_state_id))
    }
}
