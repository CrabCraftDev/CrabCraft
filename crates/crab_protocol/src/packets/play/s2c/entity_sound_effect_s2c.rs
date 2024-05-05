use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::position::Position;
use crate::types::sound::{SoundCategory, SoundEffect};
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
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
