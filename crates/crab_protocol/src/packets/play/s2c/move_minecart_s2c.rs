use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::position::Position;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "move_minecart_along_track", id = 0x31, state = Play)]
pub struct MoveMinecartS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub entity_id: i32,
    pub lerp_steps: Vec<MoveMinecartLerpStep>,
}

#[derive(ProtocolSerializer, Debug, Clone)]
pub struct MoveMinecartLerpStep {
    pub position: Position,
    pub delta_movement: Position,
    pub yaw: f32,
    pub pitch: f32,
    pub weight: f32,
}
