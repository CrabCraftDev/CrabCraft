use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x34, state = Play)]
pub struct PingResponseS2CPacket {
    pub id: i32,
}
