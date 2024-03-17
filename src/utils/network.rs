use std::io::{Read, Write};

use bytes::Bytes;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use tokio::io::AsyncReadExt;

use crate::network::packet_byte_buf::read_byte_buf::ReadByteBuf;
use crate::utils::errors::Error;

pub const CONTINUE_BIT: i32 = 0x80;
pub const SEGMENT_BITS: i32 = 0x7F;

pub fn to_var_int(mut value: i32) -> Vec<u8> {
    let mut bytes: Vec<u8> = vec![];
    loop {
        if (value & !SEGMENT_BITS) == 0 {
            bytes.push(value as u8);
            return bytes;
        }

        bytes.push(((value & SEGMENT_BITS) | CONTINUE_BIT) as u8);

        value >>= 7;
    }
}

pub async fn get_var_int_from_stream(stream: &mut (impl AsyncReadExt + Unpin)) -> Result<i32, Error> {
    let mut value: i32 = 0;
    let mut position: i8 = 0;

    loop {
        let current_byte = stream.read_u8().await?;

        value |= (current_byte as i32 & SEGMENT_BITS) << position;
        if (current_byte as i32 & CONTINUE_BIT) == 0 {
            break;
        }

        position += 7;
        if position >= 32 {
            return Err(Error::VarIntTooBig);
        }
    }

    Ok(value)
}

#[allow(dead_code)]
pub async fn decompress_packet(mut bytes: Bytes) -> Result<Bytes, Error> {
    let mut new_bytes = Vec::new();
    let data_length = bytes.get_var_int();
    if data_length == 0 {
        return Ok(Bytes::from(bytes.get_bytes(data_length)));
    }

    let compressed = &*bytes.get_bytes(data_length);
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
