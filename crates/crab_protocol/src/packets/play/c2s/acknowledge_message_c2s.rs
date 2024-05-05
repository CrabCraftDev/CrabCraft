use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x03, state = Play)]
pub struct AcknowledgeMessageC2SPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub message_count: VarInt,
}
