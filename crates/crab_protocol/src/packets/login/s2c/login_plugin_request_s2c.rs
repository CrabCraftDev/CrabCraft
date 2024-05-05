use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::bytes::EndBytes;
use crate::types::identifier::Identifier;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x04, state = Login)]
pub struct LoginPluginRequestS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub message_id: i32,
    pub channel: Identifier,
    #[protocol_attribute(bounded = 1048576)]
    pub data: EndBytes,
}
