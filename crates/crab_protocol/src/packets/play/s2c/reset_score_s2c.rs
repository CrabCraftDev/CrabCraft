use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "reset_score", id = 0x48, state = Play)]
pub struct ResetScoreS2CPacket {
    pub entity_name: String,
    pub objective_name: Option<String>,
}
