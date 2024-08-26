use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::namespaced_key::NamespacedKey;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x22, state = Play)]
pub struct PlaceRecipeC2SPacket {
    pub window_id: i8,
    pub recipe: NamespacedKey,
    pub make_all: bool,
}
