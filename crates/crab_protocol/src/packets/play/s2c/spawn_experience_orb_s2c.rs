use crab_protocol::protocol::ProtocolState::*;
use crab_protocol::types::position::Position;
use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::Packet;

#[derive(Debug, Packet, Clone)]
#[packet(id = 0x02, state = Play)]
pub struct SpawnExperienceOrbS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub entity_id: i32,
    pub position: Position,
    pub count: i16,
}
