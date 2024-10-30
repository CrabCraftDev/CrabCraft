use crab_macros::ProtocolSerializer;

use crate::packet::Packet;
use crate::protocol_state::ProtocolState;
use crate::protocol_state::ProtocolState::*;

#[derive(ProtocolSerializer, Debug, Clone)]
pub struct KeepAliveS2CPacket {
    pub keep_alive_id: i64,
}

impl Packet for KeepAliveS2CPacket {
    const STATE_IDS: &'static [(ProtocolState, i32)] = &[(Play, 0x27), (Configuration, 0x04)];
}
