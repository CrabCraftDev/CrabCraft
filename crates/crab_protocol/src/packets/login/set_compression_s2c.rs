use crab_protocol::protocol::ProtocolState::*;
use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::Packet;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x03, state = Login)]
pub struct SetCompressionS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub threshold: i32,
}
