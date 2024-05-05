use bytes::Buf;
use bytes::BufMut;

use crab_protocol::protocol_serializer::ProtocolSerializer;

use crate::error::ProtocolError;

use super::CONTINUE_BIT;
use super::SEGMENT_BITS;

pub trait WriteByteBuf: BufMut + Sized {
    fn put_var_int(&mut self, mut value: i32) {
        loop {
            if (value & !SEGMENT_BITS) == 0 {
                self.put_i8(value as i8);
                return;
            }

            self.put_i8(((value & SEGMENT_BITS) | CONTINUE_BIT) as i8);

            value >>= 7;
        }
    }

    fn put_var_long(&mut self, mut value: i64) {
        loop {
            if (value & !(SEGMENT_BITS as i64)) == 0 {
                self.put_i8(value as i8);
                return;
            }

            self.put_i8(((value & SEGMENT_BITS as i64) | CONTINUE_BIT as i64) as i8);

            value >>= 7;
        }
    }

    fn put_string(&mut self, data: &str) {
        let bytes = data.as_bytes();
        self.put_byte_array(bytes)
    }

    fn put_bool(&mut self, value: bool) {
        self.put_i8(if value { 1 } else { 0 });
    }

    fn put_raw_bytes(&mut self, value: impl Buf) {
        self.put(value)
    }

    fn put_byte_array(&mut self, value: impl Buf) {
        self.put_var_int(value.remaining() as i32);
        self.put(value);
    }

    fn put_array<T: ProtocolSerializer>(&mut self, values: &Vec<T>) -> Result<(), ProtocolError> {
        self.put_var_int(values.len() as i32);
        for val in values {
            val.serialize(self)?;
        }
        Ok(())
    }

    fn put_optional<T: ProtocolSerializer>(&mut self, value: &Option<T>) -> Result<(), ProtocolError> {
        match value {
            Some(value) => {
                self.put_bool(true);
                value.serialize(self)?;
            }
            None => self.put_bool(false),
        }
        Ok(())
    }
}

impl<T: BufMut> WriteByteBuf for T {}
