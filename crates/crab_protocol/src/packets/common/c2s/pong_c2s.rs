use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "pong", ids(Play = 0x2C, Configuration = 0x05))]
pub struct PongC2SPacket {
    pub id: i32,
}
