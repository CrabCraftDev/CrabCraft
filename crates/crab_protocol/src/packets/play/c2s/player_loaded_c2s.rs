use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "player_loaded", id = 0x2B, state = Play)]
pub struct PlayerLoadedC2SPacket;
