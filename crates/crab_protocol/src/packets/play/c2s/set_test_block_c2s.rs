use strum::FromRepr;

use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::block_position::BlockPosition;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "set_test_block", id = 0x3A, state = Play)]
pub struct SetTestBlockC2SPacket {
    pub position: BlockPosition,
    pub mode: TestBlockMode,
    pub message: String,
}

#[derive(Debug, Copy, Clone, PartialEq, ProtocolSerializer, FromRepr)]
pub enum TestBlockMode {
    Start,
    Log,
    Fail,
    Accept,
}
