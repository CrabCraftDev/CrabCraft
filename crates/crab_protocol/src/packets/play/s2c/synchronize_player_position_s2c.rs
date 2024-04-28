use bitfield_struct::bitfield;

use crab_protocol::buf::{ReadByteBuf, WriteByteBuf};
use crab_protocol::error::ProtocolError;
use crab_protocol::protocol_type::ProtocolType;
use crab_protocol::types::position::Position;
use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x3E, state = Play)]
pub struct SynchronizePlayerPositionS2CPacket {
    pub position: Position,
    pub yaw: f32,
    pub pitch: f32,
    pub flags: SynchronizePositionFlags,
    #[protocol_attribute(protocol_type = VarInt)]
    pub teleport_id: i32,
}

#[bitfield(u8)]
pub struct SynchronizePositionFlags {
    pub x: bool,
    pub y: bool,
    pub z: bool,
    pub pitch: bool,
    pub yaw: bool,
    #[bits(3)]
    __: u8,
}

impl ProtocolType for SynchronizePositionFlags {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        buf.put_i8(self.0 as i8);
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        Ok(SynchronizePositionFlags(buf.get_i8() as u8))
    }
}
