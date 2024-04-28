use crab_protocol::protocol::ProtocolState::*;
use crab_protocol::types::bit_set::FixedBitSet;
use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::Packet;

use crate::types::bytes::FixedBytes;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x05, state = Play)]
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
