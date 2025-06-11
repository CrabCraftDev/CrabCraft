use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "take_item_entity", id = 0x75, state = Play)]
pub struct PickupItemS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub collected_entity_id: i32,
    #[protocol_attribute(protocol_type = VarInt)]
    pub collector_entity_id: i32,
    #[protocol_attribute(protocol_type = VarInt)]
    pub pickup_item_count: i32,
}
