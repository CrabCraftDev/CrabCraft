use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "keep_alive", ids(Play = 0x26, Configuration = 0x04))]
pub struct KeepAliveS2CPacket {
    pub keep_alive_id: i64,
}
