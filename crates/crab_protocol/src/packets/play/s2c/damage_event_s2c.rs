use crab_protocol::protocol::ProtocolState::*;
use crab_protocol::types::position::Position;
use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::Packet;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x19, state = Play)]
pub struct DamageEventS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub entity_id: i32,
    #[protocol_attribute(protocol_type = VarInt)]
    pub source_type_id: i32,
    #[protocol_attribute(protocol_type = VarInt)]
    pub source_cause_id: i32,
    #[protocol_attribute(protocol_type = VarInt)]
    pub source_direct_id: i32,
    pub source_position: Option<Position>,
}
