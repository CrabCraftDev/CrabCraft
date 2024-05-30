use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::bytes::FixedBytes;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x1C, state = Play)]
pub struct DeleteMessageS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub message_id: i32,
    pub source_type_id: Option<FixedBytes<256>>,
}
