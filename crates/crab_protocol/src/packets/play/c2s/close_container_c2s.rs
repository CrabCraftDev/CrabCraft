use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x0F, state = Play)]
pub struct CloseContainerC2SPacket {
    pub window_id: u8,
}
