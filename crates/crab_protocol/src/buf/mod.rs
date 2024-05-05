use bytes::{BufMut, Bytes, BytesMut};
use tokio::io::AsyncReadExt;

pub use read::*;
pub use write::*;

use crate::error::ProtocolError;

pub(crate) mod read;
pub(crate) mod write;

const CONTINUE_BIT: i32 = 0x80;
const SEGMENT_BITS: i32 = 0x7F;

pub fn to_var_int(mut value: i32) -> Bytes {
    let mut bytes = BytesMut::new();
    loop {
        if (value & !SEGMENT_BITS) == 0 {
            bytes.put_i8(value as i8);
            return bytes.freeze();
        }

        bytes.put_i8(((value & SEGMENT_BITS) | CONTINUE_BIT) as i8);

        value >>= 7;
    }
}

pub async fn get_var_int_from_stream(stream: &mut (impl AsyncReadExt + Unpin)) -> Result<i32, ProtocolError> {
    let mut value: i32 = 0;
    let mut position: i8 = 0;

    loop {
        let current_byte = stream.read_i8().await?;

        value |= (current_byte as i32 & SEGMENT_BITS) << position;
        if (current_byte as i32 & CONTINUE_BIT) == 0 {
            break;
        }

        position += 7;
        if position >= 32 {
            return Err(ProtocolError::TooBigVarInt(32, position + 1));
        }
    }

    Ok(value)
}
