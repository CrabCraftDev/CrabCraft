use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x04, state = Configuration)]
pub struct KeepAliveS2CPacket {
    pub keep_alive_id: i64,
}
