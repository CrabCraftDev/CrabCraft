use crab_protocol::protocol::ProtocolState::*;
use crab_protocol_macros::Packet;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x0C, state = Play)]
pub struct ClickContainerButtonC2SPacket {
    pub window_id: i8,
    pub button_id: i8,
}
