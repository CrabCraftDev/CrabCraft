use crab_protocol::protocol::ProtocolState::*;
use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::Packet;

#[derive(Debug, Packet, Clone)]
#[packet(id = 0x05, state = Play)]
pub struct AcknowledgeBlockChangeS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub sequence_id: i32,
}
