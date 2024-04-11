use crate::protocol::ProtocolState::*;

use crab_protocol_macros::Packet;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x00, state = Status)]
pub struct StatusResponseS2CPacket {
    pub json_response: String,
}
