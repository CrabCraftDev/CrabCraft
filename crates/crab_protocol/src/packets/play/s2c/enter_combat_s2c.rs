use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x39, state = Play)]
/// Unused by the Minecraft client.
pub struct EnterCombatS2CPacket;
