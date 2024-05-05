use crab_protocol::error::ProtocolError;
use crab_protocol::protocol_serializer::ProtocolSerializer;

use crate::buf::{ReadByteBuf, WriteByteBuf};

impl<T: ProtocolSerializer> ProtocolSerializer for Option<T> {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        buf.put_optional(self)
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        buf.get_optional()
    }
}
