use uuid::Uuid;

use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x3D, state = Play)]
pub struct PlayerInfoRemoveS2CPacket {
    pub players_to_remove: Vec<Uuid>,
}
