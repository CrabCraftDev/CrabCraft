use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x1A, state = Play)]
pub struct SetPlayerOnGroundC2SPacket {
    pub on_ground: bool,
}
