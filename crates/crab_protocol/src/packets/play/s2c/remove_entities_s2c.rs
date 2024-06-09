use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x42, state = Play)]
pub struct RemoveEntitiesS2CPacket {
    pub entities_ids: Vec<i32>,
}
