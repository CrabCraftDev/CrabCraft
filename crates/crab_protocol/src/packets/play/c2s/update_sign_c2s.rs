use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::block_position::BlockPosition;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x32, state = Play)]
pub struct UpdateSignC2SPacket {
    pub position: BlockPosition,
    pub is_front_text: bool,
    #[protocol_attribute(bounded = 384)]
    pub first_line: String,
    #[protocol_attribute(bounded = 384)]
    pub second_line: String,
    #[protocol_attribute(bounded = 384)]
    pub third_line: String,
    #[protocol_attribute(bounded = 384)]
    pub fourth_line: String,
}
