use crab_macros::ProtocolSerializer;

use crate::packet::Packet;
use crate::protocol_state::ProtocolState;
use crate::protocol_state::ProtocolState::*;
use crate::types::bytes::EndBytes;
use crate::types::namespaced_key::NamespacedKey;

#[derive(ProtocolSerializer, Debug, Clone)]
pub struct PluginMessageC2SPacket {
    pub channel: NamespacedKey,
    #[protocol_attribute(bounded = 32767)]
    pub data: EndBytes,
}

impl Packet for PluginMessageC2SPacket {
    const STATE_IDS: &'static [(ProtocolState, i32)] = &[(Configuration, 0x02), (Play, 0x12)];
}
