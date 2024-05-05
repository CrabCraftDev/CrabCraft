use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::block_position::BlockPosition;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x32, state = Play)]
pub struct OpenSignEditorS2CPacket {
    pub position: BlockPosition,
    pub is_front_text: bool,
}
