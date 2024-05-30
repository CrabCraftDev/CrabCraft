use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::item_stack::ItemStack;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x0E, state = Play)]
pub struct ClickContainerC2SPacket {
    pub window_id: u8,
    #[protocol_attribute(protocol_type = VarInt)]
    pub state_id: i32,
    pub slot: i16,
    pub button: i8,
    #[protocol_attribute(protocol_type = VarInt)]
    pub mode: i32,
    #[protocol_attribute(bounded = 128)]
    pub changed_slots: Vec<ChangedSlot>,
    pub carried_item: Option<ItemStack>,
}

#[derive(Debug, Clone, PartialEq, ProtocolSerializer)]
pub struct ChangedSlot {
    pub slot_number: i16,
    pub slot_data: Option<ItemStack>,
}
