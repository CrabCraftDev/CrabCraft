use crab_protocol::types::identifier::Identifier;
use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x35, state = Play)]
pub struct PlaceGhostRecipeS2CPacket {
    pub window_id: i8,
    pub recipe: Identifier,
}
