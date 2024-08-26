use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::identifier::Identifier;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x29, state = Play)]
pub struct SetSeenRecipeC2SPacket {
    pub recipe: Identifier,
}
