use bitfield_struct::bitfield;
use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::position::{Position, Rotation, Vec3Double};
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "player_position", id = 0x42, state = Play)]
pub struct PlayerPositionS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub teleport_id: i32,
    pub position: Position,
    pub delta_movement: Vec3Double,
    pub rotation: Rotation,
    pub position_flags: PlayerPositionFlags,
}

#[bitfield(u32)]
#[derive(ProtocolSerializer)]
pub struct PlayerPositionFlags {
    pub x: bool,
    pub y: bool,
    pub z: bool,
    pub pitch: bool,
    pub yaw: bool,
    pub delta_x: bool,
    pub delta_y: bool,
    pub delta_z: bool,
    pub delta_rotation: bool,
    #[bits(23)]
    __: u32,
}
