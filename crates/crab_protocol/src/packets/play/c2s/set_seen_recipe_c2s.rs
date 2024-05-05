use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::identifier::Identifier;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x26, state = Play)]
pub struct SetSeenRecipeC2SPacket {
    pub recipe: Identifier,
}
