use crab_protocol::protocol::ProtocolState::*;
use crab_protocol_macros::Packet;

#[derive(Debug, Packet, Clone)]
#[packet(id = 0x0F, state = Play)]
pub struct ClearTitlesS2CPacket {
    pub should_reset: bool,
}
