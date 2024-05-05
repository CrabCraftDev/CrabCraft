use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x00, state = Status)]
pub struct StatusResponseS2CPacket {
    pub json_response: String,
}
