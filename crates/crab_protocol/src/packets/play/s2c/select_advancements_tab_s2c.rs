use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::namespaced_key::NamespacedKey;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "select_advancements_tab", id = 0x4F, state = Play)]
pub struct SelectAdvancementsTabS2CPacket {
    pub namespaced_key: Option<NamespacedKey>,
}
