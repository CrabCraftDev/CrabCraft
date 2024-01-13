use bytes::BufMut;
use bytes::BytesMut;
use uuid::Uuid;

use crate::utils::network::{CONTINUE_BIT, SEGMENT_BITS};

pub trait WriteByteBuf: BufMut {
    fn put_byte(&mut self, value: u8) {
        self.put_u8(value)
    }

    fn put_unsigned_byte(&mut self, value: i8) {
        self.put_i8(value)
    }

    fn put_var_int(&mut self, mut value: i32) {
        loop {
            if (value & !SEGMENT_BITS) == 0 {
                self.put_byte(value as u8);
                return;
            }

            self.put_u8(((value & SEGMENT_BITS) | CONTINUE_BIT) as u8);

            value >>= 7;
        }
    }

    fn put_var_long(&mut self, mut value: i64) {
        loop {
            if (value & !(SEGMENT_BITS as i64)) == 0 {
                self.put_byte(value as u8);
                return;
            }

            self.put_byte(((value & SEGMENT_BITS as i64) | CONTINUE_BIT as i64) as u8);

            value >>= 7;
        }
    }

    fn put_string(&mut self, data: String) {
        let bytes = data.as_bytes();
        self.put_var_int(bytes.len() as i32);

        for byte in bytes {
            self.put_byte(*byte);
        }
    }

    fn put_boolean(&mut self, value: bool) {
        self.put_byte(if value { 1 } else { 0 });
    }

    fn put_byte_array(&mut self, value: &[u8]) {
        for ele in value {
            self.put_byte(*ele)
        }
    }

    fn put_uuid(&mut self, value: Uuid) {
        self.put_u128(value.as_u128());
    }
}

impl WriteByteBuf for Vec<u8> {}

impl WriteByteBuf for BytesMut {}
