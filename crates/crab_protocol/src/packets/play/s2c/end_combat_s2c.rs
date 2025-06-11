use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "player_combat_end", id = 0x3B, state = Play)]
/// Unused by the Minecraft client.
pub struct EndCombatS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub duration: i32,
}
