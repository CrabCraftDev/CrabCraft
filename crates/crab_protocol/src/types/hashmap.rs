use std::collections::HashMap;
use std::hash::Hash;

use crab_protocol::error::ProtocolError;
use crab_protocol::protocol_serializer::ProtocolSerializer;

use crate::buf::{ReadByteBuf, WriteByteBuf};

impl<K: ProtocolSerializer + Hash + Eq, V: ProtocolSerializer> ProtocolSerializer for HashMap<K, V> {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        buf.put_hash_map(self)?;
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        buf.get_hash_map()
    }
}
