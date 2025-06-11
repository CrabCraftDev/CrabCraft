use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::block_position::BlockPosition;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "level_event", id = 0x28, state = Play)]
pub struct WorldEventS2CPacket {
    pub event: i32,
    pub position: BlockPosition,
    pub data: i32,
    pub disable_relative_volume: bool,
}
