use crab_macros::ProtocolSerializer;

use crate::packet::Packet;
use crate::protocol_state::ProtocolState;
use crate::protocol_state::ProtocolState::*;
use crate::types::bytes::EndBytes;
use crab_text::utils::namespaced_key::NamespacedKey;

#[derive(ProtocolSerializer, Debug, Clone)]
pub struct PluginMessageS2CPacket {
    pub channel: NamespacedKey,
    #[protocol_attribute(bounded = 1048576)]
    pub data: EndBytes,
}

impl Packet for PluginMessageS2CPacket {
    const NAME: &'static str = "custom_payload";
    const STATE_IDS: &'static [(ProtocolState, i32)] = &[(Configuration, 0x01), (Play, 0x18)];
}
