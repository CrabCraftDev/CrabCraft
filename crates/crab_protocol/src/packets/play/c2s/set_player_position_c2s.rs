use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::position::Position;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x17, state = Play)]
pub struct SetPlayerPositionC2SPacket {
    /// Y is feet position
    pub position: Position,
    pub on_ground: bool,
}
