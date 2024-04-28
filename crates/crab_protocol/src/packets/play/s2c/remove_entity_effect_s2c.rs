use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x41, state = Play)]
pub struct RemoveEntityEffectS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub entity_id: i32,
    #[protocol_attribute(protocol_type = VarInt)]
    pub effect_id: i32,
}
