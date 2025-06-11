use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "entity_tag_query", id = 0x18, state = Play)]
pub struct QueryEntityTagC2SPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub transaction_id: i32,
    #[protocol_attribute(protocol_type = VarInt)]
    pub entity_id: i32,
}
