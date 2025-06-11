use crab_macros::ProtocolSerializer;
use crab_nbt::NbtTag;

use crate::packet::Packet;
use crate::protocol_state::ProtocolState;
use crate::protocol_state::ProtocolState::*;
use crab_text::utils::namespaced_key::NamespacedKey;

#[derive(ProtocolSerializer, Debug, Clone)]
pub struct CustomClickActionC2SPacket {
    pub key: NamespacedKey,
    pub nbt: Option<NbtTag>,
}

impl Packet for CustomClickActionC2SPacket {
    const NAME: &'static str = "custom_click_action";
    const STATE_IDS: &'static [(ProtocolState, i32)] = &[(Configuration, 0x08), (Play, 0x41)];
}
