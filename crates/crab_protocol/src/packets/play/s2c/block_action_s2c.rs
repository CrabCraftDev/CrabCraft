use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::block_position::BlockPosition;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x08, state = Play)]
pub struct BlockActionS2CPacket {
    pub position: BlockPosition,
    pub action_id: u8,
    pub action_parameter: u8,
    #[protocol_attribute(protocol_type = VarInt)]
    pub block_type: i32,
}
