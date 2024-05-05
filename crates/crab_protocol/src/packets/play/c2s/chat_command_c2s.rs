use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::bit_set::FixedBitSet;
use crate::types::bytes::FixedBytes;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x04, state = Play)]
pub struct ChatCommandC2SPacket {
    #[protocol_attribute(bounded = 256)]
    pub message: String,
    pub timestamp: i64,
    pub salt: i64,
    pub signature: Option<FixedBytes<256>>,
    #[protocol_attribute(bounded = 8)]
    pub argument_signatures: Vec<ArgumentSignature>,
    #[protocol_attribute(protocol_type = VarInt)]
    pub message_count: i32,
    pub acknowledged: FixedBitSet<20>,
}

#[derive(Debug, Clone, PartialEq, ProtocolSerializer)]
pub struct ArgumentSignature {
    #[protocol_attribute(bounded = 16)]
    pub argument_name: String,
    pub signature: FixedBytes<256>,
}
