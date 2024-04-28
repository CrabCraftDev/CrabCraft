use crab_protocol::types::position::Position;
use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;
use crate::types::sound::{SoundCategory, SoundEffect};

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x66, state = Play)]
pub struct SoundEffectS2CPacket {
    pub sound_effect: SoundEffect,
    pub position: Position,
    pub sound_category: SoundCategory,
    pub effect_position_x: i32,
    pub effect_position_y: i32,
    pub effect_position_z: i32,
    pub volume: f32,
    pub pitch: f32,
    pub seed: i64,
}
