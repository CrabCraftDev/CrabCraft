use std::{io, str::Utf8Error};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProtocolError {
    #[error("Encountered invalid string {0:?}")]
    InvalidString(#[from] Utf8Error),
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error(transparent)]
    NbtError(#[from] crab_nbt::error::Error),
    #[error("Varint exceeds 32 bits (had {0} bits)")]
    TooBigVarInt(i8),
    #[error("Varlong exceeds 64 bits (had {0} bits)")]
    TooBigVarLong(i8),
    #[error("Enum {0} failed to be constructed of ordinal {1}")]
    EnumOutOfBounds(String, usize),
    #[error("A key in a HashMap is already present!")]
    HashMapDuplicateKey,
    #[error("Invalid identifier {0}")]
    InvalidIdentifier(String),
    #[error("Invalid block position coordinate {0}. {1} doesn't fit in {2} bits")]
    InvalidBlockPos(String, i32, u32),
    #[error("Invalid length in fixed bytes! Expected {0} but found {1}")]
    InvalidFixedBytesLength(usize, usize),
    #[error("Invalid length in fixed bit set! Expected {0} but found {1}")]
    InvalidFixedBitSetLength(usize, usize),
    #[error("The provided length ({0}) exceeds the maximum allowed length ({1})")]
    InvalidBoundedLength(usize, usize),
}
