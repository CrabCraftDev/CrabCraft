use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::identifier::Identifier;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x37, state = Play)]
pub struct PlaceGhostRecipeS2CPacket {
    pub window_id: i8,
    pub recipe: Identifier,
}
