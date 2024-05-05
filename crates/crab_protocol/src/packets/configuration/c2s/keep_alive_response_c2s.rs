use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x03, state = Configuration)]
pub struct KeepAliveResponseC2SPacket {
    pub keep_alive_id: i64,
}
