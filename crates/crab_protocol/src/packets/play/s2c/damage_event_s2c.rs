use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::position::Position;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "damage_event", id = 0x19, state = Play)]
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
