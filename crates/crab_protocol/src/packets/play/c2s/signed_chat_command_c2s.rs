use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::bit_set::FixedBitSet;
use crate::types::bytes::FixedBytes;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "chat_command_signed", id = 0x06, state = Play)]
pub struct SignedChatCommandC2SPacket {
    #[protocol_attribute(bounded = 256)]
    pub command: String,
    pub timestamp: i64,
    pub salt: i64,
    #[protocol_attribute(bounded = 8)]
    pub argument_signatures: Vec<ArgumentSignature>,
    #[protocol_attribute(protocol_type = VarInt)]
    pub seen_messages_offset: i32,
    pub seen_messages_acknowledged: FixedBitSet<20>,
}

#[derive(Debug, Clone, PartialEq, ProtocolSerializer)]
pub struct ArgumentSignature {
    #[protocol_attribute(bounded = 16)]
    pub argument_name: String,
    pub signature: FixedBytes<256>,
}
