use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x6E, state = Play)]
pub struct SetTickingStateS2CPacket {
    pub tick_rate: f32,
    pub is_frozen: bool,
}
