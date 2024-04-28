use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x4D, state = Play)]
pub struct SetBorderSizeS2CPacket {
    pub diameter: f64,
}
