use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "recipe_book_seen_recipe", id = 0x2B, state = Play)]
pub struct SetSeenRecipeC2SPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub recipe: i32,
}
