use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x05, state = Configuration)]
pub struct PingS2CPacket {
    pub id: i32,
}
