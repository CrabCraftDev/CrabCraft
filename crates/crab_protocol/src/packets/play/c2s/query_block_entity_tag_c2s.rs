use crab_protocol::protocol::ProtocolState::*;
use crab_protocol::types::block_position::BlockPosition;
use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::Packet;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x01, state = Play)]
pub struct QueryBlockEntityTagC2SPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub transaction_id: i32,
    pub position: BlockPosition,
}
