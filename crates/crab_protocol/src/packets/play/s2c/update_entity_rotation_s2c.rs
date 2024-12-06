use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::angle::Angle;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "move_entity_rot", id = 0x32, state = Play)]
pub struct UpdateEntityRotationS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub entity_id: i32,
    pub new_yaw: Angle,
    pub new_pitch: Angle,
    pub on_ground: bool,
}
