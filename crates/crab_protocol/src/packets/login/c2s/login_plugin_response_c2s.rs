use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::bytes::EndBytes;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x02, state = Login)]
pub struct LoginPluginResponseC2SPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub message_id: i32,
    #[protocol_attribute(bounded = 1048576)]
    pub data: EndBytes,
}
