use crate::error::AnvilError;
use bytes::{Buf, Bytes};
use flate2::read::{GzDecoder, ZlibDecoder};
use lz4_java_wrc::Lz4BlockInput;
use std::io::{Read, Write};
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
            AnvilCompression::LZ4 => Self::decompress_inner(Lz4BlockInput::new(&compressed[1..])),
            _ => Err(AnvilError::InvalidCompressionScheme(compression_scheme)),
        }
    }

    fn decompress_inner<R: Read>(mut decoder: R) -> Result<Bytes, AnvilError> {
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed)?;
        Ok(Bytes::from(decompressed))
    }

    pub fn compress(data: &[u8], compression: AnvilCompression) -> Result<Vec<u8>, AnvilError> {
        let mut compressed = Vec::with_capacity(data.len() + 1);
        compressed.push(compression as u8);

        match compression {
            AnvilCompression::Uncompressed => {
                compressed.extend_from_slice(data);
            }
            AnvilCompression::Gzip => {
                let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
                encoder.write_all(data)?;
                compressed.extend(encoder.finish()?);
            }
            AnvilCompression::Zlib => {
                let mut encoder = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
                encoder.write_all(data)?;
                compressed.extend(encoder.finish()?);
            }
            AnvilCompression::LZ4 => {
                let mut encoder = lz4_java_wrc::Lz4BlockOutput::new(&mut compressed);
                encoder.write_all(data)?;
            }
            _ => return Err(AnvilError::InvalidCompressionScheme(compression as u8)),
        }

        Ok(compressed)
    }
}
