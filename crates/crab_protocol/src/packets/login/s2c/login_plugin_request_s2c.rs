use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::bytes::EndBytes;
use crate::types::namespaced_key::NamespacedKey;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "custom_query", id = 0x04, state = Login)]
pub struct LoginPluginRequestS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub message_id: i32,
    pub channel: NamespacedKey,
    #[protocol_attribute(bounded = 1048576)]
    pub data: EndBytes,
}
