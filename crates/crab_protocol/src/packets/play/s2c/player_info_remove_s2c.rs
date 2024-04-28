use uuid::Uuid;

use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x3B, state = Play)]
pub struct PlayerInfoRemoveS2CPacket {
    pub players_to_remove: Vec<Uuid>,
}
