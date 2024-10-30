use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x02, state = Play)]
pub struct SelectBundleItemC2SPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub slot_id: i32,
    #[protocol_attribute(protocol_type = VarInt)]
    pub selected_item_index: i32,
}
