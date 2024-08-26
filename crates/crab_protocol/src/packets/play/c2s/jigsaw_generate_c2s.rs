use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::block_position::BlockPosition;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x17, state = Play)]
pub struct JigsawGenerateC2SPacket {
    pub position: BlockPosition,
    #[protocol_attribute(protocol_type = VarInt)]
    pub levels: i32,
    pub keep_jigsaws: bool,
}
