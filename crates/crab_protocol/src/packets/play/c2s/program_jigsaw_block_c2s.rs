use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::block_position::BlockPosition;
use crate::types::namespaced_key::NamespacedKey;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "set_jigsaw_block", id = 0x35, state = Play)]
pub struct ProgramJigsawBlockC2SPacket {
    pub position: BlockPosition,
    pub name: NamespacedKey,
    pub target: NamespacedKey,
    pub pool: NamespacedKey,
    pub final_state: String,
    pub joint_type: VarInt,
    #[protocol_attribute(protocol_type = VarInt)]
    pub selected_priority: i32,
    #[protocol_attribute(protocol_type = VarInt)]
    pub placement_priority: i32,
}
