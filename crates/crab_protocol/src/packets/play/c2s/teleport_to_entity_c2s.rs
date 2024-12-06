use uuid::Uuid;

use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "teleport_to_entity", id = 0x3B, state = Play)]
pub struct TeleportToEntityC2SPacket {
    pub target_player: Uuid,
}
