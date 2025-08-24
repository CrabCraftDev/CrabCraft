use crate::protocol_state::ProtocolState::*;
use crab_macros::{Packet, ProtocolSerializer};

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "ping", ids(Play = 0x36, Configuration = 0x05))]
pub struct PingS2CPacket {
    pub id: i32,
}
