use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::block_position::BlockPosition;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "pick_item_from_block", id = 0x23, state = Play)]
pub struct PickItemFromBlockC2SPacket {
    pub position: BlockPosition,
    pub include_data: bool,
}
