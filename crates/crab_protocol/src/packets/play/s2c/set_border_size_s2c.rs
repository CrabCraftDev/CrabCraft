use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x4F, state = Play)]
pub struct SetBorderSizeS2CPacket {
    pub diameter: f64,
}
