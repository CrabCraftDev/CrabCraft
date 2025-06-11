use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "chat_ack", id = 0x05, state = Play)]
pub struct AcknowledgeMessageC2SPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub message_count: VarInt,
}
