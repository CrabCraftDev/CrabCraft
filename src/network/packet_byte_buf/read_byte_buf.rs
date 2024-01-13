use std::str::from_utf8;

use bytes::Buf;
use bytes::Bytes;
use uuid::Uuid;

use crate::utils::network::{CONTINUE_BIT, SEGMENT_BITS};

pub trait ReadByteBuf: Buf {
    fn get_byte(&mut self) -> i8 {
        self.get_i8()
    }

    fn get_unsigned_byte(&mut self) -> u8 {
        self.get_u8()
    }

    fn get_var_int(&mut self) -> i32 {
        let mut value: i32 = 0;
        let mut position: i8 = 0;
        let mut current_byte: i8;

        loop {
            current_byte = self.get_byte();
            value |= (current_byte as i32 & SEGMENT_BITS) << position;

            if (current_byte as i32 & CONTINUE_BIT) == 0 {
                break;
            }

            position += 7;

            assert!(position < 32, "Varint exceeds 32 bits");
        }

        value
    }

    fn get_var_long(&mut self) -> i64 {
        let mut value: i64 = 0;
        let mut position: i8 = 0;
        let mut current_byte: i8;

        loop {
            current_byte = self.get_byte();
            value |= (current_byte as i64 & SEGMENT_BITS as i64) << position;

            if (current_byte as i64 & CONTINUE_BIT as i64) == 0 {
                break;
            }

            position += 7;

            assert!(position < 64);
        }

        value
    }

    fn get_string(&mut self) -> String {
        let length = self.get_var_int();
        let bytes = self.get_bytes(length);

        let str = from_utf8(bytes.as_slice()).expect("Couldn't parse string from ByteBuf!");

        str.to_string()
    }

    fn get_boolean(&mut self) -> bool {
        self.get_byte() != 0
    }

    fn get_bytes(&mut self, amount: i32) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];

        for _ in 0..amount {
            bytes.push(self.get_byte() as u8)
        }
        bytes
    }

    fn get_uuid(&mut self) -> Uuid {
        let uuid = self.get_u128();
        Uuid::from_u128(uuid)
    }
}

impl ReadByteBuf for &[u8] {}

impl ReadByteBuf for Bytes {}
