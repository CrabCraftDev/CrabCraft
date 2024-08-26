use crab_macros::ProtocolSerializer;

use crate::packet::Packet;
use crate::protocol_state::ProtocolState;
use crate::protocol_state::ProtocolState::*;

#[derive(ProtocolSerializer, Debug, Clone)]
pub struct PingRequestC2SPacket {
    pub id: i64,
}

impl Packet for PingRequestC2SPacket {
    const STATE_IDS: &'static [(ProtocolState, i32)] = &[(Play, 0x21), (Status, 0x01)];
}
