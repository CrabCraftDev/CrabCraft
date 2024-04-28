use crab_protocol::protocol::ProtocolState::*;
use crab_protocol_macros::Packet;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x1E, state = Play)]
pub struct PingRequestC2SPacket {
    pub payload: i64,
}
