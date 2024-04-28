use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x40, state = Play)]
pub struct RemoveEntitiesS2CPacket {
    pub entities_ids: Vec<i32>,
}
