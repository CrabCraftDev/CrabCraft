use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::varlong::VarLong;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x4E, state = Play)]
pub struct SetBorderLerpSizeS2CPacket {
    pub old_diameter: f64,
    pub new_diameter: f64,
    #[protocol_attribute(protocol_type = VarLong)]
    pub speed: i64,
}
