use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::angle::Angle;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "rotate_head", id = 0x4C, state = Play)]
pub struct SetHeadRotationS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub entity_id: i32,
    pub new_head_yaw: Angle,
}
