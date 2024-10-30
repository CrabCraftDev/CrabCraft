use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::text::TextComponent;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x3E, state = Play)]
pub struct CombatDeathS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub player_id: i32,
    pub message: TextComponent,
}
