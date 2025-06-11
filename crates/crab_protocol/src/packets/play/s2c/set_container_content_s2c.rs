use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::item_stack::ItemStack;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "container_set_content", id = 0x12, state = Play)]
pub struct SetContainerContentS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub window_id: i32,
    #[protocol_attribute(protocol_type = VarInt)]
    pub state_id: i32,
    pub slot_data: Vec<Option<ItemStack>>,
    pub dragged_item: Option<ItemStack>,
}
