use std::collections::HashMap;
use std::hash::Hash;

use crab_protocol::error::ProtocolError;
use crab_protocol::protocol_serializer::ProtocolSerializer;

use crate::buf::{ReadByteBuf, WriteByteBuf};

impl<K: ProtocolSerializer + Hash + Eq, V: ProtocolSerializer> ProtocolSerializer for HashMap<K, V> {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        buf.put_var_int(self.len() as i32);
        for (key, value) in self {
            key.serialize(buf)?;
            value.serialize(buf)?;
        }
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        let mut result = HashMap::new();
        let len = buf.get_var_int()?;
        for _ in 0..len {
            let key = K::deserialize(buf)?;
            if result.contains_key(&key) {
                return Err(ProtocolError::HashMapDuplicateKey);
            }
            result.insert(key, V::deserialize(buf)?);
        }
        Ok(result)
    }
}
