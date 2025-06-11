use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "player_combat_enter", id = 0x3C, state = Play)]
/// Unused by the Minecraft client.
pub struct EnterCombatS2CPacket;
