use crab_protocol::types::position::Position;
use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x18, state = Play)]
pub struct SetPlayerPositionAndRotationC2SPacket {
    /// Y is feet position
    pub position: Position,
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}
