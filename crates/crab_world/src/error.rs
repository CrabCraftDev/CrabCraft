use thiserror::Error;

use crab_protocol::error::ProtocolError;
use crab_protocol::types::namespaced_key::NamespacedKey;

#[derive(Debug, Error)]
pub enum WorldError {
    #[error("Blocks list is not initialized")]
    BlocksNotInitialized,
    #[error("Provided local id {0} exceeds maximum {1}")]
    InvalidLocalState(u16, u16),
    #[error("BlockType {0} doesn't exist!")]
    BlockDoesNotExist(NamespacedKey),
    #[error("There is not block corresponding to ID {0}")]
    BlockIdDoesNotExist(u16),
    #[error("Invalid property value")]
    InvalidPropertyValue,
    #[error("Invalid property name")]
    InvalidPropertyName,
    #[error("BlockType with id {0} was already registered")]
    DuplicateBlock(NamespacedKey),
    #[error(transparent)]
    Protocol(#[from] ProtocolError),
}
