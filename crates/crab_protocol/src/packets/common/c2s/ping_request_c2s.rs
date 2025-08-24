use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "ping_request", ids(Play = 0x25, Status = 0x01))]
pub struct PingRequestC2SPacket {
    pub id: i64,
}
