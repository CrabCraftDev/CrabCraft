use crate::packet::Packet;
use crate::protocol::{PacketDirection, ProtocolState};
use crab_protocol_macros::ProtocolSerializer;

use crate::protocol::ProtocolState::*;

#[derive(ProtocolSerializer, Debug, Clone)]
pub struct KeepAliveS2CPacket {
    pub keep_alive_id: i64,
}

impl Packet for KeepAliveS2CPacket {
    const SIDE: PacketDirection = PacketDirection::Clientbound;
    const STATE_IDS: &'static [(ProtocolState, i32)] = &[(Play, 0x26), (Configuration, 0x04)];
}
