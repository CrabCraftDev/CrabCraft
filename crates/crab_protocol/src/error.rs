use crate::protocol_state::ProtocolState;
use libdeflater::DecompressionError;
use std::{io, str::Utf8Error};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProtocolError {
    #[error("Encountered invalid string {0:?}")]
    InvalidString(#[from] Utf8Error),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Nbt(#[from] crab_nbt::error::Error),
    #[error(transparent)]
    Text(#[from] crab_text::error::TextError),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    LibdeflateDecompression(#[from] DecompressionError),
    #[error("VarInt exceeds {0} bits (had {1} bits)")]
    TooBigVarInt(usize, i8),
    #[error("Enum {0} failed to be constructed of ordinal {1}")]
    EnumOutOfBounds(String, usize),
    #[error("A key in a HashMap is already present!")]
    HashMapDuplicateKey,
    #[error("Invalid block position coordinate {0}. {1} doesn't fit in {2} bits")]
    InvalidBlockPos(String, i32, u32),
    #[error("Invalid length in fixed bytes! Expected {0} but found {1}")]
    InvalidFixedBytesLength(usize, usize),
    #[error("Invalid length in fixed bit set! Expected {0} but found {1}")]
    InvalidFixedBitSetLength(usize, usize),
    #[error("The provided length ({0}) exceeds the maximum allowed length ({1})")]
    InvalidBoundedLength(usize, usize),
    #[error("Packet {0} doesn't support state {1}!")]
    InvalidState(String, ProtocolState),
    #[error("Tried to decompress packet with different size: {0} than provided: {1}!")]
    DecompressedSizeMismatch(usize, usize),
}
