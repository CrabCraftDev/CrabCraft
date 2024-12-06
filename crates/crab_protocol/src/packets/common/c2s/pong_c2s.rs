use crab_macros::ProtocolSerializer;

use crate::packet::Packet;
use crate::protocol_state::ProtocolState;
use crate::protocol_state::ProtocolState::*;

#[derive(ProtocolSerializer, Debug, Clone)]
pub struct PongC2SPacket {
    pub id: i32,
}

impl Packet for PongC2SPacket {
    const NAME: &'static str = "pong";
    const STATE_IDS: &'static [(ProtocolState, i32)] = &[(Play, 0x29), (Configuration, 0x05)];
}
