use crab_protocol::types::position::Position;
use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x17, state = Play)]
pub struct SetPlayerPositionC2SPacket {
    /// Y is feet position
    pub position: Position,
    pub on_ground: bool,
}
