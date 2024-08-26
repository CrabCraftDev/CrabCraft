use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x3B, state = Play)]
/// Unused by the Minecraft client.
pub struct EnterCombatS2CPacket;
