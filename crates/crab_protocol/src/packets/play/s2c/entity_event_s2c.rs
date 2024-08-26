use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x1F, state = Play)]
pub struct EntityEventS2CPacket {
    pub entity_id: i32,
    pub entity_status: i8,
}
