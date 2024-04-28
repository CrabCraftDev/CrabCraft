use crab_protocol::protocol::ProtocolState::*;
use crab_protocol::types::block_position::BlockPosition;
use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::Packet;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x14, state = Play)]
pub struct JigsawGenerateC2SPacket {
    pub position: BlockPosition,
    #[protocol_attribute(protocol_type = VarInt)]
    pub levels: i32,
    pub keep_jigsaws: bool,
}
