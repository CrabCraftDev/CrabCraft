use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "pick_item_from_entity", id = 0x23, state = Play)]
pub struct PickItemFromEntityC2SPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub id: i32,
    pub include_data: bool,
}
