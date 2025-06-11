use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crab_text::utils::namespaced_key::NamespacedKey;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "select_advancements_tab", id = 0x4E, state = Play)]
pub struct SelectAdvancementsTabS2CPacket {
    pub namespaced_key: Option<NamespacedKey>,
}
