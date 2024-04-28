use strum::FromRepr;

use crab_protocol::buf::{ReadByteBuf, WriteByteBuf};
use crab_protocol::error::ProtocolError;
use crab_protocol::protocol::ProtocolState::*;
use crab_protocol::protocol_type::ProtocolType;
use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::Packet;

#[derive(Debug, Packet, Clone)]
#[packet(id = 0x03, state = Play)]
pub struct EntityAnimationS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub entity_id: i32,
    pub animation: EntityAnimation,
}

#[derive(Debug, Copy, Clone, PartialEq, FromRepr)]
#[repr(u8)]
pub enum EntityAnimation {
    SwingMainArm = 0,
    LeaveBed = 2,
    SwingOtherArm = 3,
    CriticalEffect = 4,
    MagicalCriticalEffect = 5,
}

impl ProtocolType for EntityAnimation {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        buf.put_byte(*self as i8);
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        let value = buf.get_byte() as u8;
        Self::from_repr(value).ok_or(ProtocolError::EnumOutOfBounds(
            "EntityAnimation".to_string(),
            value as usize,
        ))
    }
}
