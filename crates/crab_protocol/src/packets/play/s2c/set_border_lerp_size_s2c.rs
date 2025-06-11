use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::varlong::VarLong;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "set_border_lerp_size", id = 0x52, state = Play)]
pub struct SetBorderLerpSizeS2CPacket {
    pub old_diameter: f64,
    pub new_diameter: f64,
    #[protocol_attribute(protocol_type = VarLong)]
    pub speed: i64,
}
