use crab_protocol::protocol::ProtocolState::*;
use crab_protocol::types::block_position::BlockPosition;
use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::Packet;

#[derive(Debug, Packet, Clone)]
#[packet(id = 0x06, state = Play)]
pub struct SetBlockDestroyStageS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub breaker_entity_id: i32,
    pub position: BlockPosition,
    pub destroy_stage: i8,
}
