use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::item_stack::ItemStack;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x15, state = Play)]
pub struct SetContainerSlotS2CPacket {
    pub window_id: u8,
    #[protocol_attribute(protocol_type = VarInt)]
    pub state_id: i32,
    pub slot_id: i16,
    pub slot: Option<ItemStack>,
}
