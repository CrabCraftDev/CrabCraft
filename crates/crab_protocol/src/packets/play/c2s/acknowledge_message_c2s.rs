use crab_protocol::protocol::ProtocolState::*;
use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::Packet;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x03, state = Play)]
pub struct AcknowledgeMessageC2SPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub message_count: VarInt,
}
