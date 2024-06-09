use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::position::Position;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x1A, state = Play)]
pub struct SetPlayerPositionC2SPacket {
    /// Y is feet position
    pub position: Position,
    pub on_ground: bool,
}
