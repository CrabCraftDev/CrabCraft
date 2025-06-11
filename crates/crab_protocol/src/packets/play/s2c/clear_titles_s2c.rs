use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "clear_titles", id = 0x0E, state = Play)]
pub struct ClearTitlesS2CPacket {
    pub should_reset: bool,
}
