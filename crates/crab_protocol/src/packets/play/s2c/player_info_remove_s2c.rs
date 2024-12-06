use uuid::Uuid;

use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "player_info_remove", id = 0x3F, state = Play)]
pub struct PlayerInfoRemoveS2CPacket {
    pub players_to_remove: Vec<Uuid>,
}
