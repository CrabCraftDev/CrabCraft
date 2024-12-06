use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::item_stack::ItemStack;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "set_creative_mode_slot", id = 0x34, state = Play)]
pub struct SetCreativeModeSlotC2SPacket {
    pub slot: i16,
    pub clicked_item: Option<ItemStack>,
}
