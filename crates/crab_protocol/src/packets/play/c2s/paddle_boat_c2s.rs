use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x1C, state = Play)]
pub struct PaddleBoatC2SPacket {
    pub left_paddle_turning: bool,
    pub right_paddle_turning: bool,
}
