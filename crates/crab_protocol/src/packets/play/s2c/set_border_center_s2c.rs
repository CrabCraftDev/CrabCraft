use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x4B, state = Play)]
pub struct SetBorderCenterS2CPacket {
    pub x: f64,
    pub z: f64,
}
