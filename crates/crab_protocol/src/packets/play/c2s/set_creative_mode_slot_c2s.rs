use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::item_stack::ItemStack;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x32, state = Play)]
pub struct SetCreativeModeSlotC2SPacket {
    pub slot: i16,
    pub clicked_item: Option<ItemStack>,
}
