use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "set_experience", id = 0x60, state = Play)]
pub struct SetExperienceS2CPacket {
    pub experience_bar: f32,
    #[protocol_attribute(protocol_type = VarInt)]
    pub level: i32,
    #[protocol_attribute(protocol_type = VarInt)]
    pub total_experience: i32,
}
