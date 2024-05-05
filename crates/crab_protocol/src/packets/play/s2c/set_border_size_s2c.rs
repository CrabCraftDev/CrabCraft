use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x4D, state = Play)]
pub struct SetBorderSizeS2CPacket {
    pub diameter: f64,
}
