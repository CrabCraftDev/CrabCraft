use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::bytes::EndBytes;
use crab_text::utils::namespaced_key::NamespacedKey;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "custom_payload", ids(Configuration = 0x01, Play = 0x18))]
pub struct PluginMessageS2CPacket {
    pub channel: NamespacedKey,
    #[protocol_attribute(bounded = 1048576)]
    pub data: EndBytes,
}
