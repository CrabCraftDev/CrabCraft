use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "set_carried_item", id = 0x33, state = Play)]
pub struct SetHeldItemC2SPacket {
    pub primary_effect: i16,
}
