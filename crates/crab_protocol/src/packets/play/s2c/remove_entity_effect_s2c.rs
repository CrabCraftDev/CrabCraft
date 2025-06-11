use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "remove_mob_effect", id = 0x47, state = Play)]
pub struct RemoveEntityEffectS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub entity_id: i32,
    #[protocol_attribute(protocol_type = VarInt)]
    pub effect_id: i32,
}
