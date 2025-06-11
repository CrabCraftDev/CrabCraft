use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "remove_entities", id = 0x46, state = Play)]
pub struct RemoveEntitiesS2CPacket {
    pub entities_ids: Vec<i32>,
}
