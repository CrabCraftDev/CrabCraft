use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::block_position::BlockPosition;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x56, state = Play)]
pub struct SetDefaultSpawnPositionS2CPacket {
    pub location: BlockPosition,
    pub angle: f32,
}
