use crate::buf::{ReadByteBuf, WriteByteBuf};
use crate::error::ProtocolError;

pub trait ProtocolSerializer: Sized {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError>;
    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError>;
}
