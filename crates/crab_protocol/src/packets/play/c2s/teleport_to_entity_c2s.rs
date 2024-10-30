use uuid::Uuid;

use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x39, state = Play)]
pub struct TeleportToEntityC2SPacket {
    pub target_player: Uuid,
}
