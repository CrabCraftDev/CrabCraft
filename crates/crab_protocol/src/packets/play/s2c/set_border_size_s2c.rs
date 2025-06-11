use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "set_border_size", id = 0x53, state = Play)]
pub struct SetBorderSizeS2CPacket {
    pub diameter: f64,
}
