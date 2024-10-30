use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x47, state = Play)]
pub struct RemoveEntitiesS2CPacket {
    pub entities_ids: Vec<i32>,
}
