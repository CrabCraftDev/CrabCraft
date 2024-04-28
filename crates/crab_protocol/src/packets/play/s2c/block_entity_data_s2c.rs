use crab_nbt::Nbt;

use crab_protocol::protocol::ProtocolState::*;
use crab_protocol::types::block_position::BlockPosition;
use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::Packet;

#[derive(Debug, Packet, Clone)]
#[packet(id = 0x07, state = Play)]
pub struct BlockEntityDataS2CPacket {
    pub position: BlockPosition,
    #[protocol_attribute(protocol_type = VarInt)]
    pub block_entity_type: i32,
    pub block_entity_nbt: Nbt,
}
