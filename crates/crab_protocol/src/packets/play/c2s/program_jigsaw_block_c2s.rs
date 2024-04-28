use crab_protocol::types::block_position::BlockPosition;
use crab_protocol::types::identifier::Identifier;
use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x30, state = Play)]
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
