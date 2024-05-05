use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::position::Position;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x18, state = Play)]
pub struct SetPlayerPositionAndRotationC2SPacket {
    /// Y is feet position
    pub position: Position,
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}
