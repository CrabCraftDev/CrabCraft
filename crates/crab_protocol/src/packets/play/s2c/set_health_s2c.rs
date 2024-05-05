use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x5B, state = Play)]
pub struct SetHealthS2CPacket {
    pub health: f32,
    #[protocol_attribute(protocol_type = VarInt)]
    pub food: i32,
    pub food_saturation: f32,
}
