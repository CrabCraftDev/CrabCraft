use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x31, state = Play)]
pub struct SetHeldItemC2SPacket {
    pub primary_effect: i16,
}
