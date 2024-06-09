use uuid::Uuid;

use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::angle::Angle;
use crate::types::position::Position;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x01, state = Play)]
pub struct SpawnEntityS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub entity_id: i32,
    pub entity_uuid: Uuid,
    #[protocol_attribute(protocol_type = VarInt)]
    pub entity_type: i32,
    pub position: Position,
    pub pitch: Angle,
    pub yaw: Angle,
    pub head_yaw: Angle,
    #[protocol_attribute(protocol_type = VarInt)]
    pub data: i32,
    pub velocity_x: i16,
    pub velocity_y: i16,
    pub velocity_z: i16,
}
