use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::bytes::EndBytes;
use crab_text::utils::namespaced_key::NamespacedKey;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "custom_payload", ids(Configuration = 0x02, Play = 0x15))]
pub struct PluginMessageC2SPacket {
    pub channel: NamespacedKey,
    #[protocol_attribute(bounded = 32767)]
    pub data: EndBytes,
}
