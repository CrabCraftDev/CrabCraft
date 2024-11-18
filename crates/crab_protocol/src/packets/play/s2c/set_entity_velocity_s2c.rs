use crate::protocol_state::ProtocolState::*;
use crate::types::position::Velocity;
use crate::types::varint::VarInt;
use crab_macros::{Packet, ProtocolSerializer};

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x5F, state = Play)]
pub struct SetEntityVelocityS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub entity_id: i32,
    pub velocity: Velocity,
}
