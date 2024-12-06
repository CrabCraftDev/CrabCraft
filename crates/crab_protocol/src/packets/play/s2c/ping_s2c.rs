use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "ping", id = 0x37, state = Play)]
pub struct PingS2CPacket {
    pub id: i32,
}
