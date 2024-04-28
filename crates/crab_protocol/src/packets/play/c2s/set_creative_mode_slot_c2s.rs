use crab_protocol::types::item_stack::ItemStack;
use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x2F, state = Play)]
pub struct SetCreativeModeSlotC2SPacket {
    pub slot: i16,
    pub clicked_item: Option<ItemStack>,
}
