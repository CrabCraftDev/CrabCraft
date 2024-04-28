use crab_protocol::types::angle::Angle;
use crab_protocol::types::position::Position;
use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x6D, state = Play)]
pub struct TeleportEntityS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub entity_id: i32,
    pub position: Position,
    pub new_yaw: Angle,
    pub new_pitch: Angle,
    pub on_ground: bool,
}
