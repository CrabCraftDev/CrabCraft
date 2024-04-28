use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x33, state = Play)]
pub struct PingS2CPacket {
    pub id: i32,
}
