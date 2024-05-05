use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::angle::Angle;
use crate::types::position::Position;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x6D, state = Play)]
pub struct TeleportEntityS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub entity_id: i32,
    pub position: Position,
    pub new_yaw: Angle,
    pub new_pitch: Angle,
    pub on_ground: bool,
}
