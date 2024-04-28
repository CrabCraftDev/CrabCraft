use crab_protocol::protocol::ProtocolState::*;
use crab_protocol_macros::Packet;

#[derive(Debug, Packet, Clone)]
#[packet(id = 0x12, state = Play)]
pub struct CloseContainerS2CPacket {
    pub window_id: u8,
}
