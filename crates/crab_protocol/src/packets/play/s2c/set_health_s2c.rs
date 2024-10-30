use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x62, state = Play)]
pub struct SetHealthS2CPacket {
    pub health: f32,
    #[protocol_attribute(protocol_type = VarInt)]
    pub food: i32,
    pub food_saturation: f32,
}
