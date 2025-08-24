use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "keep_alive", ids(Configuration = 0x04, Play = 0x1B))]
pub struct KeepAliveResponseC2SPacket {
    pub keep_alive_id: i64,
}
