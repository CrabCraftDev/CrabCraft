use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;
use crate::types::angle::Angle;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x2D, state = Play)]
pub struct UpdateEntityPositionAndRotationS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub entity_id: i32,
    pub delta_x: i16,
    pub delta_y: i16,
    pub delta_z: i16,
    pub new_yaw: Angle,
    pub new_pitch: Angle,
    pub on_ground: bool,
}
