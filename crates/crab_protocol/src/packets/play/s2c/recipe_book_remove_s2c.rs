use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "recipe_book_remove", id = 0x45, state = Play)]
pub struct RecipeBookRemoveS2CPacket {
    #[protocol_attribute(protocol_type = Vec<VarInt>)]
    pub book: Vec<i32>,
}
