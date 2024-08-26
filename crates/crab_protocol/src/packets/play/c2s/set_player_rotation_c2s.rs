use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x1C, state = Play)]
pub struct SetPlayerRotationC2SPacket {
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}
