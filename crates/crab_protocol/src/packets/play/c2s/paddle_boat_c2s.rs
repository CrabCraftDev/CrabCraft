use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "paddle_boat", id = 0x22, state = Play)]
pub struct PaddleBoatC2SPacket {
    pub left_paddle_turning: bool,
    pub right_paddle_turning: bool,
}
