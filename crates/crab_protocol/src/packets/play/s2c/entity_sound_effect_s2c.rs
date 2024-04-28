use crab_protocol::types::position::Position;
use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;
use crate::types::sound::{SoundCategory, SoundEffect};

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x65, state = Play)]
pub struct EntitySoundEffectS2CPacket {
    pub sound_effect: SoundEffect,
    pub position: Position,
    pub sound_category: SoundCategory,
    #[protocol_attribute(protocol_type = VarInt)]
    pub entity_id: i32,
    pub volume: f32,
    pub pitch: f32,
    pub seed: i64,
}
