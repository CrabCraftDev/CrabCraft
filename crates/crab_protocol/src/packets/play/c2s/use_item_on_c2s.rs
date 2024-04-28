use crab_protocol::types::block_position::BlockPosition;
use crab_protocol::types::hand::Hand;
use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;
use crate::types::block_face::BlockFace;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x35, state = Play)]
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
