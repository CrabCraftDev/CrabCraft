use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::Status;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x01, state = Status)]
pub struct PingResponseS2CPacket {
    pub payload: i64,
}
