use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::varint::VarInt;
use crab_text::utils::namespaced_key::NamespacedKey;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "place_ghost_recipe", id = 0x38, state = Play)]
pub struct PlaceGhostRecipeS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub window_id: i32,
    pub recipe: NamespacedKey,
}
