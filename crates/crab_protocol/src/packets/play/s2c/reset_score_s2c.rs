use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x42, state = Play)]
pub struct ResetScoreS2CPacket {
    pub entity_name: String,
    pub objective_name: Option<String>,
}
