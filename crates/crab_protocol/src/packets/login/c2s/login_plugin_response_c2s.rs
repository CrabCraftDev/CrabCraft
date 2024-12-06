use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::bytes::EndBytes;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "custom_query_answer", id = 0x02, state = Login)]
pub struct LoginPluginResponseC2SPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub message_id: i32,
    #[protocol_attribute(bounded = 1048576)]
    pub data: EndBytes,
}
