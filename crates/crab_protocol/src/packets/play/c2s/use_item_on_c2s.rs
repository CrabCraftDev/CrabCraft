use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::block_face::BlockFace;
use crate::types::block_position::BlockPosition;
use crate::types::hand::Hand;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "use_item_on", id = 0x3A, state = Play)]
pub struct UseItemOnC2SPacket {
    pub hand: Hand,
    pub block_position: BlockPosition,
    pub face: BlockFace,
    pub cursor_position_x: f32,
    pub cursor_position_y: f32,
    pub cursor_position_z: f32,
    pub is_player_head_inside_block: bool,
    #[protocol_attribute(protocol_type = VarInt)]
    pub sequence: i32,
}
