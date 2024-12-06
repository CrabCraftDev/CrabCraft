use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::position::Position;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "entity_position_sync", id = 0x20, state = Play)]
pub struct EntityPositionSyncS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub entity_id: i32,
    pub position: Position,
    pub delta_movement: Position,
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}
