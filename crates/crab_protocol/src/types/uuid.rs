use uuid::Uuid;

use crab_protocol::protocol_serializer::ProtocolSerializer;

use crate::buf::{ReadByteBuf, WriteByteBuf};
use crate::error::ProtocolError;

impl ProtocolSerializer for Uuid {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        buf.put_u128(self.as_u128());
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        Ok(Uuid::from_u128(buf.get_u128()))
    }
}
