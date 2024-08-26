use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x1D, state = Play)]
pub struct SetPlayerOnGroundC2SPacket {
    pub on_ground: bool,
}
