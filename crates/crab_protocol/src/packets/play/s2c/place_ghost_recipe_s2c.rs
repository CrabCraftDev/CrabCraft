use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::namespaced_key::NamespacedKey;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "place_ghost_recipe", id = 0x39, state = Play)]
pub struct PlaceGhostRecipeS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub window_id: i32,
    pub recipe: NamespacedKey,
}
