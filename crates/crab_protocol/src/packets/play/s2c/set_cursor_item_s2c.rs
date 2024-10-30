use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::item_stack::ItemStack;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x5A, state = Play)]
pub struct SetCursorItemS2CPacket {
    pub contents: ItemStack,
}
