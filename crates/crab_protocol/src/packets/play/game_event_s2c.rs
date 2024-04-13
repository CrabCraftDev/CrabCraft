use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x20, state = Play)]
pub struct GameEventS2CPacket {
    pub event: u8,
    pub value: f32,
}
