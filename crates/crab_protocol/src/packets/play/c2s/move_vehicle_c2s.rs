use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::position::{Position, Rotation};

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "move_vehicle", id = 0x21, state = Play)]
pub struct MoveVehicleC2SPacket {
    pub position: Position,
    pub rotation: Rotation,
}
