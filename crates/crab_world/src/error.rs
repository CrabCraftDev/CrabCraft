use std::io;
use thiserror::Error;

use crab_protocol::error::ProtocolError;
use crab_text::error::TextError;
use crab_text::utils::namespaced_key::NamespacedKey;

#[derive(Debug, Error)]
pub enum WorldError {
    #[error("Blocks registry has not been initialized")]
    BlocksNotInitialized,
    #[error("Provided local id {0} exceeds maximum {1}")]
    InvalidLocalState(u16, u16),
    #[error("Block entity with id {0} doesn't exist!")]
    BlockDoesNotExist(NamespacedKey),
    #[error("There is not block corresponding to ID {0}")]
    BlockIdDoesNotExist(u16),
    #[error("Invalid property value")]
    InvalidPropertyValue,
    #[error("Invalid property name")]
    InvalidPropertyName,
    #[error("BlockType with id {0} was already registered")]
    DuplicateBlock(NamespacedKey),

    #[error("Index outside of bounds! Index: {0}, Length: {1}")]
    IndexOutsideOfBounds(usize, usize),
    #[error("Value {0} exceeds bit mask {1}")]
    ValueExceedsMask(i64, i64),
    #[error("Missing block in chunk section at {0} {1} {2}")]
    MissingBlock(u8, u8, u8),
    #[error("Invalid block position inside chunk section {0} {1} {2}")]
    InvalidPositionInChunkSection(u8, u8, u8),

    #[error(transparent)]
    Protocol(#[from] ProtocolError),
}

#[derive(Debug, Error)]
pub enum AnvilError {
    #[error("Invalid world file compression scheme {0}")]
    InvalidCompressionScheme(u8),
    #[error(transparent)]
    IO(#[from] io::Error),
    #[error("Chunk {x} {z} has not been generated yet")]
    ChunkNotGenerated { x: u8, z: u8 },
    #[error("NBT tag not found")]
    NbtTagNotFound,
    #[error("BlockType {0} doesn't exist!")]
    BlockEntityDoesNotExist(NamespacedKey),

    #[error(transparent)]
    Protocol(#[from] ProtocolError),
    #[error(transparent)]
    Text(#[from] TextError),
}
