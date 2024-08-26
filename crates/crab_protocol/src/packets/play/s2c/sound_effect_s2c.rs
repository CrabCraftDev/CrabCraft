use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::sound::{SoundCategory, SoundEffect};

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x68, state = Play)]
pub struct SoundEffectS2CPacket {
    pub sound_effect: SoundEffect,
    pub sound_category: SoundCategory,
    pub effect_position_x: i32,
    pub effect_position_y: i32,
    pub effect_position_z: i32,
    pub volume: f32,
    pub pitch: f32,
    pub seed: i64,
}
