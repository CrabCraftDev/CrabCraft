use std::io;
use thiserror::Error;

use crab_nbt::error::Error as NbtError;
use crab_protocol::error::ProtocolError;
use crab_text::error::TextError;
use crab_text::utils::namespaced_key::NamespacedKey;

#[derive(Debug, Error)]
pub enum BlockError {
    #[error("Invalid property value")]
    InvalidPropertyValue,
    #[error("Invalid property name")]
    InvalidPropertyName,

    #[error("Blocks registry has not been initialized")]
    BlocksNotInitialized,
    #[error("Provided local id {0} exceeds maximum {1}")]
    InvalidLocalState(u16, u16),
    #[error("Block entity with id {0} doesn't exist!")]
    BlockDoesNotExist(NamespacedKey),
    #[error("There is not block corresponding to ID {0}")]
    BlockIdDoesNotExist(u16),
    #[error("BlockType with id {0} was already registered")]
    DuplicateBlock(NamespacedKey),
}

#[derive(Debug, Error)]
pub enum WorldError {
    #[error("Attempted to set index {0} in BitStorage of size {1}")]
    BitStorageSetOutOfRange(usize, usize),
    #[error("Value {0} exceeds bit mask {1}")]
    ValueExceedsMask(i64, i64),
    #[error("Missing block in chunk section at {0} {1} {2}")]
    MissingBlock(u8, u8, u8),
    #[error("Invalid block position inside chunk section {0} {1} {2}")]
    InvalidPositionInChunkSection(u8, u8, u8),

    #[error("Missing world file {0}")]
    MissingWorldFile(String),
    #[error("Failed to load chunk ({x}, {z})")]
    FailedToLoadChunk { x: i32, z: i32 },
    #[error("Trying to access not loaded chunk at {x}, {z}")]
    AccessUnloadedChunk { x: i32, z: i32 },
    #[error("Invalid position in chunk")]
    InvalidPositionInChunk,

    #[error(transparent)]
    Protocol(#[from] ProtocolError),
    #[error(transparent)]
    Block(#[from] BlockError),
    #[error(transparent)]
    Anvil(#[from] AnvilError),
}

#[derive(Debug, Error)]
pub enum AnvilError {
    #[error("Invalid world file compression scheme {0}")]
    InvalidCompressionScheme(u8),
    #[error(transparent)]
    IO(#[from] io::Error),
    #[error("Chunk at {x} {z} does not exist")]
    ChunkDoesNotExist { x: u8, z: u8 },
    #[error("Invalid chunk size: {size} bytes. Expected between 1 byte and 1 MiB.")]
    InvalidChunkSize { size: usize },
    #[error("NBT tag not found")]
    NbtTagNotFound,
    #[error("BlockType {0} doesn't exist!")]
    BlockEntityDoesNotExist(NamespacedKey),

    #[error(transparent)]
    Protocol(#[from] ProtocolError),
    #[error(transparent)]
    Text(#[from] TextError),
    #[error(transparent)]
    Nbt(#[from] NbtError),
    #[error(transparent)]
    Block(#[from] BlockError),
}
