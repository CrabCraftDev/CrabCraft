use crab_protocol::types::block_position::BlockPosition;
use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x54, state = Play)]
pub struct SetDefaultSpawnPositionS2CPacket {
    pub location: BlockPosition,
    pub angle: f32,
}
