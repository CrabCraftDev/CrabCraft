use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::position::Position;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x1E, state = Play)]
pub struct MoveVehicleC2SPacket {
    pub position: Position,
    pub yaw: f32,
    pub pitch: f32,
}
