use std::cmp::PartialEq;
use std::collections::HashMap;

use crate::blocks::block_data::BlockData;
use crate::blocks::block_registry::BlockInformation;
use crate::blocks::{BLOCKS, get_block_registry};
use crate::error::BlockError;
use crab_text::utils::namespaced_key::NamespacedKey;
use derive_more::{From, Into};

/// Represents a block used in our codebase
/// u16 is block state id (mostly used in protocol), which is unique id for each block state
#[derive(Debug, Clone, Copy, Eq, PartialEq, Into, From)]
#[repr(transparent)]
pub struct Block(pub u16);

impl Block {
    pub fn namespaced_key(&self) -> Result<&NamespacedKey, BlockError> {
        BLOCKS
            .get()
            .ok_or(BlockError::BlocksNotInitialized)?
            .namespaced_key_by_state_id(self.0)
    }

    pub fn properties(&self) -> Result<HashMap<&'static str, String>, BlockError> {
        let namespaced_key = self.namespaced_key()?;
        let block_information = get_block_registry()?.block_information_by_namespaced_key(namespaced_key)?;
        let local_state = self.0 - block_information.starting_state_id;
        let properties = (block_information.properties)(local_state)?;

        Ok(properties)
    }

    pub fn properties2(&self) -> Result<(&NamespacedKey, HashMap<&'static str, String>), BlockError> {
        let namespaced_key = self.namespaced_key()?;
        let block_information = get_block_registry()?.block_information_by_namespaced_key(namespaced_key)?;
        let local_state = self.0 - block_information.starting_state_id;
        let properties = (block_information.properties)(local_state)?;

        Ok((namespaced_key, properties))
    }

    /// Creates a new Block based on Hashmap<&str, String>
    pub fn from_properties(
        namespaced_key: &NamespacedKey,
        properties: &HashMap<&str, String>,
    ) -> Result<Block, BlockError> {
        let block_information = get_block_registry()?.block_information_by_namespaced_key(namespaced_key)?;
        (block_information.from_properties)(properties)
    }

    pub fn is_same_as(&self, other: &Block) -> bool {
        matches!((self.namespaced_key(), other.namespaced_key()), (Ok(own_key), Ok(other_key)) if own_key == other_key)
    }

    pub const fn is_air(&self) -> bool {
        Self::is_air_i64(self.0 as i64)
    }

    /// Checking for air is executed so many times in (count_non_air_blocks)[crate::chunks::section::ChunkSection::count_non_air_blocks] function
    /// So it requires i64 instead of Block/u16 to avoid casting
    pub const fn is_air_i64(block: i64) -> bool {
        // For efficiency, those are hardcoded IDs of air, cave air and void air blocks
        matches!(block, 0 | 14013 | 14014)
    }

    pub fn property<P: BlockData>(&self) -> Result<P::Value, BlockError> {
        P::from_properties(self.properties()?).map(P::get)
    }

    pub fn apply_property<P: BlockData>(&mut self, property: &P::Value) -> Result<(), BlockError> {
        let mut properties = self.properties()?;
        P::apply(property, &mut properties)?;
        let new_block = Self::from_properties(self.namespaced_key()?, &properties)?;
        self.0 = new_block.0;
        Ok(())
    }
}

/// Represents generated block (for example StoneBlock)
/// With methods for parsing its protocol ID
pub trait BlockType {
    /// Returns unique NamespacedKey of block
    /// For example `minecraft:stone`
    fn namespaced_key() -> NamespacedKey;

    fn properties(local_state: u16) -> Result<HashMap<&'static str, String>, BlockError>;
    fn from_properties(hashmap: &HashMap<&str, String>) -> Result<Block, BlockError>;

    fn is_same_type_as(block: &Block) -> bool {
        matches!(block.namespaced_key(), Ok(block_key) if *block_key == Self::namespaced_key())
    }

    fn information() -> Result<&'static BlockInformation, BlockError> {
        let blocks = get_block_registry()?;
        blocks.block_information_by_namespaced_key(&Self::namespaced_key())
    }

    fn default_state_id() -> Result<Block, BlockError> {
        Ok(Block(Self::information()?.default_state_id))
    }

    fn starting_state_id() -> Result<Block, BlockError> {
        Ok(Block(Self::information()?.starting_state_id))
    }
}
