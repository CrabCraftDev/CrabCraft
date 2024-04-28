use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x2C, state = Play)]
pub struct SetHeldItemC2SPacket {
    pub primary_effect: i16,
}
