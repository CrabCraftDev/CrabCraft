use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x59, state = Play)]
pub struct LinkEntitiesS2CPacket {
    pub attached_entity_id: i32,
    /// Set -1 to detach
    pub holding_entity_id: i32,
}
