use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x23, state = Play)]
pub struct InitializeWorldBorderS2CPacket {
    pub x: f64,
    pub z: f64,
    pub previous_diameter: f64,
    pub new_diameter: f64,
    #[protocol_attribute(protocol_type = VarInt)]
    pub speed: i32,
    #[protocol_attribute(protocol_type = VarInt)]
    pub portal_teleport_boundary: i32,
    #[protocol_attribute(protocol_type = VarInt)]
    pub warning_blocks: i32,
    #[protocol_attribute(protocol_type = VarInt)]
    pub warning_time: i32,
}
