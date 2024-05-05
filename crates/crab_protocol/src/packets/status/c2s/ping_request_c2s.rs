use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x01, state = Status)]
pub struct PingRequestC2SPacket {
    pub payload: i64,
}
