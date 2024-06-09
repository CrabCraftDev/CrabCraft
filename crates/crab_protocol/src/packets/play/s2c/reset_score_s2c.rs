use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x44, state = Play)]
pub struct ResetScoreS2CPacket {
    pub entity_name: String,
    pub objective_name: Option<String>,
}
