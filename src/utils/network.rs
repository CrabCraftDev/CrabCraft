use std::io::{Read, Write};

use bytes::Bytes;
use crab_protocol::buf::{to_var_int, ReadByteBuf};
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;

use crate::utils::errors::Error;

#[allow(dead_code)]
pub async fn decompress_packet(mut bytes: Bytes) -> Result<Bytes, Error> {
    let mut new_bytes = Vec::new();
    let data_length = bytes.get_var_int()?;
    if data_length == 0 {
        return Ok(bytes.get_raw_bytes(data_length));
    }

    let compressed = &*bytes.get_raw_bytes(data_length);
    let mut decoder = ZlibDecoder::new(compressed);
    decoder.read_to_end(&mut new_bytes)?;

    Ok(Bytes::from(new_bytes))
}

#[allow(dead_code)]
pub async fn compress_packet(bytes: Bytes, compression_threshold: i32) -> Result<Bytes, Error> {
    let should_compress = bytes.len() as i32 > compression_threshold;
    let mut data_length = to_var_int(if should_compress { 0 } else { bytes.len() as i32 });

    let mut compressed;
    if should_compress {
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&bytes)?;
        compressed = encoder.finish()?;
    } else {
        compressed = Vec::from(bytes)
    }

    let mut packet: Vec<u8> = vec![];
    packet.append(&mut to_var_int((data_length.len() + compressed.len()) as i32));
    packet.append(&mut data_length);
    packet.append(&mut compressed);

    Ok(Bytes::from(packet))
}
