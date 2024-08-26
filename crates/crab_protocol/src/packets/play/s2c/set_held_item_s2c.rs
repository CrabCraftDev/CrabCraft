use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x53, state = Play)]
pub struct SetHeldItemS2CPacket {
    pub slot: i8,
}
