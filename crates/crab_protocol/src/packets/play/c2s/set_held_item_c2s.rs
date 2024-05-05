use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x2C, state = Play)]
pub struct SetHeldItemC2SPacket {
    pub primary_effect: i16,
}
