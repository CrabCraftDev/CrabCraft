use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::block_position::BlockPosition;
use crate::types::identifier::Identifier;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x33, state = Play)]
pub struct ProgramJigsawBlockC2SPacket {
    pub position: BlockPosition,
    pub name: Identifier,
    pub target: Identifier,
    pub pool: Identifier,
    pub final_state: String,
    pub joint_type: VarInt,
    #[protocol_attribute(protocol_type = VarInt)]
    pub selected_priority: i32,
    #[protocol_attribute(protocol_type = VarInt)]
    pub placement_priority: i32,
}
