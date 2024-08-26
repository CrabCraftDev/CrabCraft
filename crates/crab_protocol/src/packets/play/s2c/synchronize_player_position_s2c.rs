use bitfield_struct::bitfield;

use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::position::Position;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x40, state = Play)]
pub struct SynchronizePlayerPositionS2CPacket {
    pub position: Position,
    pub yaw: f32,
    pub pitch: f32,
    pub flags: SynchronizePositionFlags,
    #[protocol_attribute(protocol_type = VarInt)]
    pub teleport_id: i32,
}

#[bitfield(u8)]
#[derive(ProtocolSerializer)]
pub struct SynchronizePositionFlags {
    pub x: bool,
    pub y: bool,
    pub z: bool,
    pub pitch: bool,
    pub yaw: bool,
    #[bits(3)]
    __: u8,
}
