use crate::packet::Packet;
use crate::protocol::{PacketDirection, ProtocolState};
use crab_protocol_macros::ProtocolSerializer;

use crate::protocol::ProtocolState::*;

#[derive(ProtocolSerializer, Debug, Clone)]
pub struct KeepAliveResponseC2SPacket {
    pub keep_alive_id: i64,
}

impl Packet for KeepAliveResponseC2SPacket {
    const SIDE: PacketDirection = PacketDirection::Serverbound;
    const STATE_IDS: &'static [(ProtocolState, i32)] = &[(Configuration, 0x04), (Play, 0x18)];
}
