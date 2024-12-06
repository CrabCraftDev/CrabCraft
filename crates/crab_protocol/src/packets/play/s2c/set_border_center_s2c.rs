use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "set_border_center", id = 0x52, state = Play)]
pub struct SetBorderCenterS2CPacket {
    pub x: f64,
    pub z: f64,
}
