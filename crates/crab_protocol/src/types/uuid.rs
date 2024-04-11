use crate::buf::{ReadByteBuf, WriteByteBuf};
use crate::error::ProtocolError;
use crab_protocol::protocol_type::ProtocolType;
use uuid::Uuid;

impl ProtocolType for Uuid {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        buf.put_u128(self.as_u128());
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        Ok(Uuid::from_u128(buf.get_u128()))
    }
}
