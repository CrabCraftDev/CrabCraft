use crab_protocol::protocol::ProtocolState::*;
use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::Packet;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x12, state = Play)]
pub struct QueryEntityTagC2SPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub transaction_id: i32,
    #[protocol_attribute(protocol_type = VarInt)]
    pub entity_id: i32,
}
