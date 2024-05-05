use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::block_position::BlockPosition;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x06, state = Play)]
pub struct SetBlockDestroyStageS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub breaker_entity_id: i32,
    pub position: BlockPosition,
    pub destroy_stage: i8,
}
