use crab_protocol::protocol::ProtocolState::*;
use crab_protocol::types::block_position::BlockPosition;
use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::Packet;

#[derive(Debug, Packet, Clone)]
#[packet(id = 0x08, state = Play)]
pub struct BlockActionS2CPacket {
    pub position: BlockPosition,
    pub action_id: u8,
    pub action_parameter: u8,
    #[protocol_attribute(protocol_type = VarInt)]
    pub block_type: i32,
}
