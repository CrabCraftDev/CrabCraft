use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x12, state = Play)]
pub struct CloseContainerS2CPacket {
    pub window_id: u8,
}
