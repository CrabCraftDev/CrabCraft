use crate::protocol::ProtocolState::*;
use crate::types::varint::VarInt;

use crab_protocol_macros::Packet;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x3E, state = Play)]
pub struct SynchronizePlayerPositionS2CPacket {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub flags: i8,
    #[protocol_attribute(protocol_type = VarInt)]
    pub teleport_id: i32,
}
