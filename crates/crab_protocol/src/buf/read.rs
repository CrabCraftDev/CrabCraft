use std::collections::HashMap;
use std::hash::Hash;
use std::str::from_utf8;

use bytes::{Buf, Bytes};

use crate::error::ProtocolError;
use crate::protocol_serializer::ProtocolSerializer;

use super::CONTINUE_BIT;
use super::SEGMENT_BITS;

pub trait ReadByteBuf: Buf + Sized {
    fn get_byte(&mut self) -> i8 {
        self.get_i8()
    }

    fn get_unsigned_byte(&mut self) -> u8 {
        self.get_u8()
    }

    fn get_var_int(&mut self) -> Result<i32, ProtocolError> {
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

            if position > 31 {
                return Err(ProtocolError::TooBigVarInt(position + 1));
            }
        }

        Ok(value)
    }

    fn get_var_long(&mut self) -> Result<i64, ProtocolError> {
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

            if position > 63 {
                return Err(ProtocolError::TooBigVarLong(position + 1));
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
        self.get_byte() != 0
    }

    fn get_raw_bytes(&mut self, length: i32) -> Bytes {
        self.copy_to_bytes(length as usize)
    }

    fn get_byte_array(&mut self) -> Result<Bytes, ProtocolError> {
        let length = self.get_var_int()?;
        let bytes = self.get_raw_bytes(length);
        Ok(bytes)
    }

    fn get_some_array<T: ProtocolSerializer>(&mut self) -> Result<Vec<T>, ProtocolError> {
        let length = self.get_var_int()?;
        let mut items = Vec::with_capacity(length as usize);

        for _i in 0..length {
            items.push(T::deserialize(self)?);
        }

        Ok(items)
    }

    fn get_optional<T: ProtocolSerializer>(&mut self) -> Result<Option<T>, ProtocolError> {
        let present = self.get_bool();
        match present {
            true => Ok(Some(T::deserialize(self)?)),
            false => Ok(None),
        }
    }

    fn get_hash_map<K: ProtocolSerializer + Hash + Eq, V: ProtocolSerializer>(
        &mut self,
    ) -> Result<HashMap<K, V>, ProtocolError> {
        let mut result = HashMap::new();
        let len = self.get_var_int()?;
        for _ in 0..len {
            let key = K::deserialize(self)?;
            if result.contains_key(&key) {
                return Err(ProtocolError::HashMapDuplicateKey);
            }
            result.insert(key, V::deserialize(self)?);
        }
        Ok(result)
    }
}

impl<T: Buf> ReadByteBuf for T {}
