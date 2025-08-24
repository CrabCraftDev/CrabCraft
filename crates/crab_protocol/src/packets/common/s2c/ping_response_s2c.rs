use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "pong_response", ids(Play = 0x37, Status = 0x01))]
pub struct PingResponseS2CPacket {
    pub id: i64,
}
