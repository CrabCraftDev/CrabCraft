use crab_protocol::buf::{ReadByteBuf, WriteByteBuf};
use crab_protocol::error::ProtocolError;
use crab_protocol::protocol_serializer::ProtocolSerializer;
use crab_text::utils::namespaced_key::NamespacedKey;

impl ProtocolSerializer for NamespacedKey {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        ProtocolSerializer::serialize(&self.to_string(), buf)?;
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        let value: String = ProtocolSerializer::deserialize(buf)?;
        Ok(value.parse()?)
    }
}
