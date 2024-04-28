use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x34, state = Play)]
pub struct PingResponseS2CPacket {
    pub id: i32,
}
