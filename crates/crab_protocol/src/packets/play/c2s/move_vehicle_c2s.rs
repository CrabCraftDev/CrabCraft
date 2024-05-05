use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::position::Position;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x1B, state = Play)]
pub struct MoveVehicleC2SPacket {
    pub position: Position,
    pub yaw: f32,
    pub pitch: f32,
}
