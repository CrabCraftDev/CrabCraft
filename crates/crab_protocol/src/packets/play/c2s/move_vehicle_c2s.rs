use crab_protocol::types::position::Position;
use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x1B, state = Play)]
pub struct MoveVehicleC2SPacket {
    pub position: Position,
    pub yaw: f32,
    pub pitch: f32,
}
