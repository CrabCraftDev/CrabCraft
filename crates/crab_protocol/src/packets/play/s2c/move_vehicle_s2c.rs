use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::angle::Angle;
use crate::types::position::Position;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x33, state = Play)]
pub struct MoveVehicleS2CPacket {
    pub position: Position,
    pub yaw: Angle,
    pub pitch: Angle,
}
