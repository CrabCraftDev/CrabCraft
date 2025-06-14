use crate::error::AnvilError;
use bytes::Bytes;
use flate2::read::{GzDecoder, ZlibDecoder};
use lz4::Decoder;
use std::io::Read;
use strum::FromRepr;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, FromRepr)]
#[repr(u8)]
pub enum Compression {
    Gzip = 1,
    #[default]
    Zlib = 2,
    Uncompressed = 3,
    LZ4 = 4,
    Custom = 127,
}

impl Compression {
    pub fn decompress(&self, compressed: Bytes) -> Result<Bytes, AnvilError> {
        match self {
            Compression::Uncompressed => Ok(compressed),
            Compression::Gzip => Self::decompress_inner(GzDecoder::new(compressed.as_ref())),
            Compression::Zlib => Self::decompress_inner(ZlibDecoder::new(compressed.as_ref())),
            Compression::LZ4 => Self::decompress_inner(Decoder::new(compressed.as_ref())?),
            scheme => Err(AnvilError::InvalidCompressionScheme(*scheme as u8)),
        }
    }

    fn decompress_inner<R: Read>(mut decoder: R) -> Result<Bytes, AnvilError> {
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed)?;
        Ok(Bytes::from(decompressed))
    }
}
