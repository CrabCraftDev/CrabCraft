use crab_nbt::Nbt;

use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::block_position::BlockPosition;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x07, state = Play)]
pub struct BlockEntityDataS2CPacket {
    pub position: BlockPosition,
    #[protocol_attribute(protocol_type = VarInt)]
    pub block_entity_type: i32,
    pub block_entity_nbt: Nbt,
}
