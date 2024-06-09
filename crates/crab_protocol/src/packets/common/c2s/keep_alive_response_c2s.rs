use crab_protocol_macros::ProtocolSerializer;

use crate::packet::Packet;
use crate::protocol_state::ProtocolState;
use crate::protocol_state::ProtocolState::*;

#[derive(ProtocolSerializer, Debug, Clone)]
pub struct KeepAliveResponseC2SPacket {
    pub keep_alive_id: i64,
}

impl Packet for KeepAliveResponseC2SPacket {
    const STATE_IDS: &'static [(ProtocolState, i32)] = &[(Configuration, 0x04), (Play, 0x18)];
}
