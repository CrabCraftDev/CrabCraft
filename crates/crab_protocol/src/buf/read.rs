use std::str::from_utf8;

use bytes::{Buf, Bytes};

use crate::error::ProtocolError;
use crate::protocol_serializer::ProtocolSerializer;

use super::CONTINUE_BIT;
use super::SEGMENT_BITS;

pub trait ReadByteBuf: Buf + Sized {
    fn get_var_int(&mut self) -> Result<i32, ProtocolError> {
        let mut value: i32 = 0;
        let mut position: i8 = 0;
        let mut current_byte: i8;

        loop {
            current_byte = self.get_i8();
            value |= (current_byte as i32 & SEGMENT_BITS) << position;

            if (current_byte as i32 & CONTINUE_BIT) == 0 {
                break;
            }

            position += 7;

            if position > 31 {
                return Err(ProtocolError::TooBigVarInt(32, position + 1));
            }
        }

        Ok(value)
    }

    fn get_var_long(&mut self) -> Result<i64, ProtocolError> {
        let mut value: i64 = 0;
        let mut position: i8 = 0;
        let mut current_byte: i8;

        loop {
            current_byte = self.get_i8();
            value |= (current_byte as i64 & SEGMENT_BITS as i64) << position;

            if (current_byte as i64 & CONTINUE_BIT as i64) == 0 {
                break;
            }

            position += 7;

            if position > 63 {
                return Err(ProtocolError::TooBigVarInt(64, position + 1));
            }
        }

        Ok(value)
    }

    fn get_string(&mut self) -> Result<String, ProtocolError> {
        let bytes = self.get_byte_array()?;
        let str = from_utf8(bytes.chunk())?;
        Ok(str.to_string())
    }

    fn get_bool(&mut self) -> bool {
        self.get_i8() != 0
    }

    fn get_raw_bytes(&mut self, length: i32) -> Bytes {
        self.copy_to_bytes(length as usize)
    }

    fn get_byte_array(&mut self) -> Result<Bytes, ProtocolError> {
        let length = self.get_var_int()?;
        let bytes = self.get_raw_bytes(length);
        Ok(bytes)
    }

    fn get_array<T: ProtocolSerializer>(&mut self) -> Result<Vec<T>, ProtocolError> {
        let length = self.get_var_int()?;
        let mut items = Vec::with_capacity(length as usize);

        for _i in 0..length {
            items.push(T::deserialize(self)?);
        }

        Ok(items)
    }

    fn get_optional<T: ProtocolSerializer>(&mut self) -> Result<Option<T>, ProtocolError> {
        match self.get_bool() {
            true => Ok(Some(T::deserialize(self)?)),
            false => Ok(None),
        }
    }
}

impl<T: Buf> ReadByteBuf for T {}
