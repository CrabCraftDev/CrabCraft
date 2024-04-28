use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x15, state = Play)]
pub struct KeepAliveResponseC2SPacket {
    pub keep_alive_id: i64,
}
