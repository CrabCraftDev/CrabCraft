use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::game_mode::GameMode;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "change_game_mode", id = 0x04, state = Play)]
pub struct ChangeGameModeC2SPacket {
    pub game_mode: GameMode,
}
