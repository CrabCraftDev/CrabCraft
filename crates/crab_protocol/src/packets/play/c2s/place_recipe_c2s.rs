use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "place_recipe", id = 0x24, state = Play)]
pub struct PlaceRecipeC2SPacket {
    pub window_id: i8,
    #[protocol_attribute(protocol_type = VarInt)]
    pub recipe: i32,
    pub make_all: bool,
}
