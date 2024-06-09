use crab_protocol_macros::ProtocolSerializer;

use crate::packet::Packet;
use crate::protocol_state::ProtocolState;
use crate::protocol_state::ProtocolState::*;

#[derive(ProtocolSerializer, Debug, Clone)]
pub struct PingResponseS2CPacket {
    pub id: i64,
}

impl Packet for PingResponseS2CPacket {
    const STATE_IDS: &'static [(ProtocolState, i32)] = &[(Play, 0x36), (Status, 0x01)];
}
