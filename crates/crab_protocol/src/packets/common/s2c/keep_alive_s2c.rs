use crab_macros::ProtocolSerializer;

use crate::packet::Packet;
use crate::protocol_state::ProtocolState;
use crate::protocol_state::ProtocolState::*;

#[derive(ProtocolSerializer, Debug, Clone)]
pub struct KeepAliveS2CPacket {
    pub keep_alive_id: i64,
}

impl Packet for KeepAliveS2CPacket {
    const NAME: &'static str = "keep_alive";
    const STATE_IDS: &'static [(ProtocolState, i32)] = &[(Play, 0x26), (Configuration, 0x04)];
}
