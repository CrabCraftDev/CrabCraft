use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "set_passengers", id = 0x64, state = Play)]
pub struct SetPassengersS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub entity_id: i32,
    #[protocol_attribute(protocol_type = Vec<VarInt>)]
    pub passengers: Vec<i32>,
}
