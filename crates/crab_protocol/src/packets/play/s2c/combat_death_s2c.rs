use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::varint::VarInt;
use crab_text::text::Text;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "player_combat_kill", id = 0x3D, state = Play)]
pub struct CombatDeathS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub player_id: i32,
    pub message: Text,
}
