use uuid::Uuid;

use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x37, state = Play)]
pub struct TeleportToEntityC2SPacket {
    pub target_player: Uuid,
}
