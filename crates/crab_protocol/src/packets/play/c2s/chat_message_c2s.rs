use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::bit_set::FixedBitSet;
use crate::types::bytes::FixedBytes;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x06, state = Play)]
pub struct ChatMessageC2SPacket {
    #[protocol_attribute(bounded = 256)]
    pub message: String,
    pub timestamp: i64,
    pub salt: i64,
    pub signature: Option<FixedBytes<256>>,
    #[protocol_attribute(protocol_type = VarInt)]
    pub message_count: i32,
    pub acknowledged: FixedBitSet<20>,
}
