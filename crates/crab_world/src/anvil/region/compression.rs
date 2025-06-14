use crate::error::AnvilError;
use bytes::{Buf, Bytes};
use flate2::read::{GzDecoder, ZlibDecoder};
use lz4::Decoder;
use std::io::Read;
use strum::FromRepr;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, FromRepr)]
#[repr(u8)]
pub enum AnvilCompression {
    Gzip = 1,
    #[default]
    Zlib = 2,
    Uncompressed = 3,
    LZ4 = 4,
    Custom = 127,
}

impl AnvilCompression {
    pub fn decompress(compressed: Vec<u8>) -> Result<Bytes, AnvilError> {
        let compression_scheme = compressed[0];
        let compression = AnvilCompression::from_repr(compression_scheme)
            .ok_or(AnvilError::InvalidCompressionScheme(compression_scheme))?;

        match compression {
            AnvilCompression::Uncompressed => {
                let mut bytes = Bytes::from(compressed);
                bytes.advance(1);
                Ok(bytes)
            }
            AnvilCompression::Gzip => Self::decompress_inner(GzDecoder::new(&compressed[1..])),
            AnvilCompression::Zlib => Self::decompress_inner(ZlibDecoder::new(&compressed[1..])),
            AnvilCompression::LZ4 => Self::decompress_inner(Decoder::new(compressed)),
            _ => Err(AnvilError::InvalidCompressionScheme(compression_scheme)),
        }
    }

    fn decompress_inner<R: Read>(mut decoder: R) -> Result<Bytes, AnvilError> {
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed)?;
        Ok(Bytes::from(decompressed))
    }
}
