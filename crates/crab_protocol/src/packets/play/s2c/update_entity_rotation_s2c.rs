use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;
use crate::types::angle::Angle;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x2E, state = Play)]
pub struct UpdateEntityRotationS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub entity_id: i32,
    pub new_yaw: Angle,
    pub new_pitch: Angle,
    pub on_ground: bool,
}
