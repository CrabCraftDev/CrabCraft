use crab_protocol::buf::{ReadByteBuf, WriteByteBuf};
use crab_protocol::error::ProtocolError;
use crab_protocol::protocol_type::ProtocolType;
use strum::FromRepr;

#[derive(Debug, Copy, Clone, PartialEq, FromRepr)]
#[repr(u8)]
pub enum BlockFace {
    Bottom,
    Top,
    North,
    South,
    West,
    East,
}

impl ProtocolType for BlockFace {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        buf.put_byte(*self as i8);
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        let value = buf.get_byte() as u8;
        Self::from_repr(value).ok_or(ProtocolError::EnumOutOfBounds("BlockFace".to_string(), value as usize))
    }
}
