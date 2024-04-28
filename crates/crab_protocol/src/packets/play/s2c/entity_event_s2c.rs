use crab_protocol::protocol::ProtocolState::*;
use crab_protocol_macros::Packet;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x1D, state = Play)]
pub struct EntityEventS2CPacket {
    pub entity_id: i32,
    pub entity_status: i8,
}
