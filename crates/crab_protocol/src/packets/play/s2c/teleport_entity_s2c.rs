use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::angle::Angle;
use crate::types::position::Position;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "teleport_entity", id = 0x77, state = Play)]
pub struct TeleportEntityS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub entity_id: i32,
    pub position: Position,
    pub new_yaw: Angle,
    pub new_pitch: Angle,
    pub on_ground: bool,
}
