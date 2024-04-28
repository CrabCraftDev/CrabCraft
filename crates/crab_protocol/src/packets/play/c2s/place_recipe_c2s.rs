use crab_protocol::protocol::ProtocolState::*;
use crab_protocol::types::identifier::Identifier;
use crab_protocol_macros::Packet;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x1F, state = Play)]
pub struct PlaceRecipeC2SPacket {
    pub window_id: i8,
    pub recipe: Identifier,
    pub make_all: bool,
}
