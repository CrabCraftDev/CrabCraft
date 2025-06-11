use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::block_position::BlockPosition;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "open_sign_editor", id = 0x35, state = Play)]
pub struct OpenSignEditorS2CPacket {
    pub position: BlockPosition,
    pub is_front_text: bool,
}
