use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x5C, state = Play)]
pub struct SetExperienceS2CPacket {
    pub experience_bar: f32,
    #[protocol_attribute(protocol_type = VarInt)]
    pub level: i32,
    #[protocol_attribute(protocol_type = VarInt)]
    pub total_experience: i32,
}
