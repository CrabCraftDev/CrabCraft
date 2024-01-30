use std::error::Error;

use tokio::io::AsyncReadExt;

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

pub async fn get_var_int_from_stream(stream: &mut (impl AsyncReadExt + Unpin)) -> Result<i32, Box<dyn Error + Send>> {
    let mut value: i32 = 0;
    let mut position: i8 = 0;

    loop {
        let current_byte = match stream.read_u8().await {
            Ok(byte) => byte,
            Err(e) => return Err(Box::new(e)),
        };

        value |= (current_byte as i32 & SEGMENT_BITS) << position;
        if (current_byte as i32 & CONTINUE_BIT) == 0 {
            break;
        }

        position += 7;

        assert!(position < 32, "Varint exceeds 32 bits");
    }

    Ok(value)
}
