use crab_protocol::types::block_position::BlockPosition;
use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
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
