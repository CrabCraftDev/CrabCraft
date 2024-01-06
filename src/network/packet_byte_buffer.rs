use bytes::BufMut;
use bytes::Buf;

pub trait Packet {
    fn get_id(&self) -> u8;
}
pub trait ReadByteBuffer : Buf {
    fn read_var_int(&mut self) -> i32;
}

const CONTINUE_BIT: i32 = 0x80;
const SEGMENT_BITS: i32 = 0x7F;
impl dyn ReadByteBuffer {
    #[allow(dead_code)]
    fn read_var_int(&mut self) -> i32 {
        let mut value: i32 = 0;
        let mut position = 0;
        let mut current_byte: i8;

        loop {
            current_byte = self.get_i8();
            value |= (current_byte as i32 & SEGMENT_BITS) << position;

            if (current_byte as i32 & CONTINUE_BIT) == 0 {
                break
            }

            position += 7;
            
            assert!(position < 32);
        }

        return value;
    }
}

pub trait WriteByteBuffer : BufMut {
    fn put_var_int(&mut self, value: i32);
    fn put_string(&mut self, data: String);
}

impl WriteByteBuffer for Vec<u8> {
    fn put_var_int(&mut self, mut value: i32) {
        loop {
            if (value & !SEGMENT_BITS) == 0 {
                self.put_u8(value as u8);
                return;
            }

            self.put_u8(((value & SEGMENT_BITS) | CONTINUE_BIT) as u8);

            value >>= 7;
        }
    }

    fn put_string(&mut self, data: String) {
        let bytes = data.as_bytes();
        self.put_var_int(bytes.len() as i32);

        for byte in bytes {
            self.put_u8(*byte);
        }
    }
}

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
