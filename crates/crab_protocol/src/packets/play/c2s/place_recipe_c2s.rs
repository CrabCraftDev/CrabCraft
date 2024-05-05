use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::identifier::Identifier;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x1F, state = Play)]
pub struct PlaceRecipeC2SPacket {
    pub window_id: i8,
    pub recipe: Identifier,
    pub make_all: bool,
}
