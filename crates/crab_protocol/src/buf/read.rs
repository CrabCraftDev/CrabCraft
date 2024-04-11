use std::collections::HashMap;
use std::hash::Hash;
use std::str::from_utf8;

use bytes::{Buf, Bytes};
use uuid::Uuid;

use crab_protocol::protocol_type::ProtocolType;

use crate::error::ProtocolError;

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

    fn get_boolean(&mut self) -> bool {
        self.get_byte() != 0
    }

    fn get_raw_bytes(&mut self, amount: i32) -> Bytes {
        self.copy_to_bytes(amount as usize)
    }

    fn get_byte_array(&mut self) -> Result<Bytes, ProtocolError> {
        let length = self.get_var_int()?;
        let bytes = self.get_raw_bytes(length);
        Ok(bytes)
    }

    fn get_some_array<T: ProtocolType>(&mut self) -> Result<Vec<T>, ProtocolError> {
        let length = self.get_var_int()?;
        let mut items = Vec::with_capacity(length as usize);

        for _i in 0..length {
            items.push(T::deserialize(self)?);
        }

        Ok(items)
    }

    fn get_optional<T: ProtocolType>(&mut self) -> Result<Option<T>, ProtocolError> {
        let present = self.get_boolean();
        match present {
            true => Ok(Some(T::deserialize(self)?)),
            false => Ok(None),
        }
    }

    fn get_hash_map<K: ProtocolType + Hash + Eq, V: ProtocolType>(&mut self) -> Result<HashMap<K, V>, ProtocolError> {
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

impl<T: Buf> ReadByteBuf for T {}
