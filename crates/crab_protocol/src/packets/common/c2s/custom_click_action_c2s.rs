use crab_macros::{Packet, ProtocolSerializer};
use crab_nbt::NbtTag;

use crate::protocol_state::ProtocolState::*;
use crab_text::utils::namespaced_key::NamespacedKey;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "custom_click_action", ids(Configuration = 0x08, Play = 0x41))]
pub struct CustomClickActionC2SPacket {
    pub key: NamespacedKey,
    pub nbt: Option<NbtTag>,
}
