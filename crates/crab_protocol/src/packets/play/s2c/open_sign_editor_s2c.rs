use crab_protocol::protocol::ProtocolState::*;
use crab_protocol::types::block_position::BlockPosition;
use crab_protocol_macros::Packet;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x32, state = Play)]
pub struct OpenSignEditorS2CPacket {
    pub position: BlockPosition,
    pub is_front_text: bool,
}
