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
