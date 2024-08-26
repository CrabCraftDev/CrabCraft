use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::position::Position;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x02, state = Play)]
pub struct SpawnExperienceOrbS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub entity_id: i32,
    pub position: Position,
    pub count: i16,
}
