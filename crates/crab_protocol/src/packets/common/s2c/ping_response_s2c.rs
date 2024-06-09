use crate::protocol::{PacketDirection, ProtocolState};
use crab_protocol_macros::ProtocolSerializer;

use crate::packet::Packet;
use crate::protocol::ProtocolState::*;

#[derive(ProtocolSerializer, Debug, Clone)]
pub struct PingResponseS2CPacket {
    pub id: i64,
}

impl Packet for PingResponseS2CPacket {
    const SIDE: PacketDirection = PacketDirection::Clientbound;
    const STATE_IDS: &'static [(ProtocolState, i32)] = &[(Play, 0x36), (Status, 0x01)];
}
