use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::block_position::BlockPosition;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x26, state = Play)]
pub struct WorldEventS2CPacket {
    pub event: i32,
    pub position: BlockPosition,
    pub data: i32,
    pub disable_relative_volume: bool,
}
