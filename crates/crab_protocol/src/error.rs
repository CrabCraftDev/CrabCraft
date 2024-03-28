use std::{io, num::TryFromIntError, str::Utf8Error};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProtocolError {
    #[error("Failed to convert int to other type")]
    FromIntErr(#[from] TryFromIntError),
    #[error("Encountered invalid string {0:?}")]
    InvalidString(#[from] Utf8Error),
    #[error("Varint exceeds 32 bits (had {0} bits)")]
    TooBigVarInt(i8),
    #[error("Varlong exceeds 64 bits (had {0} bits)")]
    TooBigVarLong(i8),
    #[error("Got packet with unknown id {0}")]
    UnknownId(i32),
    #[error("Unable to read byte array with unknown length")]
    ByteArrayLengthUnknown,
    #[error("This feature is not yet implemented: {0}")]
    UnimplementedFeature(String),
    #[error("Plugin message has too much data ({0}), should be max 1048576 bytes")]
    PluginMessageTooBig(usize),
    #[error("Enum {0} failed to be constructed of ordinal {1}")]
    EnumOutOfBounds(String, usize),
    #[error("A key in a HashMap is already present!")]
    HashMapDuplicateKey,
    #[error(transparent)]
    IoError(#[from] io::Error),
}
