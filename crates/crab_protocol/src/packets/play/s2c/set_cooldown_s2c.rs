use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x17, state = Play)]
pub struct SetCooldownS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub item_id: i32,
    #[protocol_attribute(protocol_type = VarInt)]
    pub cooldown_ticks: i32,
}
