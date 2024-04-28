use crab_protocol::types::varlong::VarLong;
use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x4C, state = Play)]
pub struct SetBorderLerpSizeS2CPacket {
    pub old_diameter: f64,
    pub new_diameter: f64,
    #[protocol_attribute(protocol_type = VarLong)]
    pub speed: i64,
}
