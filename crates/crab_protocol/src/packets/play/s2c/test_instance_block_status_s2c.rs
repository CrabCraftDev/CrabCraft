use crate::protocol_state::ProtocolState::*;
use crate::types::position::Vec3Int;
use crab_macros::{Packet, ProtocolSerializer};
use crab_text::text::Text;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "test_instance_block_status", id = 0x77, state = Play)]
pub struct TestInstanceBlockStatusS2CPacket {
    pub status: Text,
    pub size: Option<Vec3Int>,
}
