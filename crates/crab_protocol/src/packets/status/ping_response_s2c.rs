use crab_protocol::protocol::ProtocolState::Status;
use crab_protocol_macros::Packet;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x01, state = Status)]
pub struct PingResponseS2CPacket {
    pub payload: i64,
}
