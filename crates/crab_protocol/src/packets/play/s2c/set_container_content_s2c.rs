use crab_protocol::protocol::ProtocolState::*;
use crab_protocol::types::item_stack::ItemStack;
use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::Packet;

#[derive(Debug, Packet, Clone)]
#[packet(id = 0x13, state = Play)]
pub struct SetContainerContentS2CPacket {
    pub window_id: u8,
    #[protocol_attribute(protocol_type = VarInt)]
    pub state_id: i32,
    pub slot_data: Vec<Option<ItemStack>>,
    pub dragged_item: Option<ItemStack>,
}
