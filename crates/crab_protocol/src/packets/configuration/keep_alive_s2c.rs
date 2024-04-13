use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x03, state = Configuration)]
pub struct KeepAliveS2CPacket {
    pub keep_alive_id: i64,
}
