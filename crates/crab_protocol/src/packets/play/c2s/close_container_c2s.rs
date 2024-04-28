use crab_protocol::protocol::ProtocolState::*;
use crab_protocol_macros::Packet;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x0E, state = Play)]
pub struct CloseContainerC2SPacket {
    pub window_id: u8,
}
