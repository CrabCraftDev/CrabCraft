use crab_protocol::protocol::ProtocolState::*;
use crab_protocol::types::bytes::FixedBytes;
use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::Packet;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x1A, state = Play)]
pub struct DeleteMessageS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub message_id: i32,
    pub source_type_id: Option<FixedBytes<256>>,
}
