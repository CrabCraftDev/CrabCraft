use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x19, state = Play)]
pub struct SetPlayerRotationC2SPacket {
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}
