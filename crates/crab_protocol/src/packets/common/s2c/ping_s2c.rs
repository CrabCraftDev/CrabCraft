use crate::protocol_state::ProtocolState::*;
use crab_macros::ProtocolSerializer;
use crab_protocol::packet::Packet;
use crab_protocol::protocol_state::ProtocolState;

#[derive(ProtocolSerializer, Debug, Clone)]
pub struct PingS2CPacket {
    pub id: i32,
}

impl Packet for PingS2CPacket {
    const NAME: &'static str = "ping";
    const STATE_IDS: &'static [(ProtocolState, i32)] = &[(Play, 0x37), (Configuration, 0x05)];
}
