use bitfield_struct::bitfield;

use crab_macros::{Packet, ProtocolSerializer};

use crate::buf::{ReadByteBuf, WriteByteBuf};
use crate::error::ProtocolError;
use crate::protocol_serializer::ProtocolSerializer;
use crate::protocol_state::ProtocolState::*;
use crate::types::namespaced_key::NamespacedKey;
use crate::types::sound::SoundCategory;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x71, state = Play)]
pub struct StopSoundS2CPacket {
    pub sound_category: Option<SoundCategory>,
    pub sound_name: Option<NamespacedKey>,
}

impl ProtocolSerializer for StopSoundS2CPacket {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        let mut stop_sound_controls = StopSoundControls::new();
        stop_sound_controls.set_category(self.sound_category.is_some());
        stop_sound_controls.set_name(self.sound_name.is_some());
        buf.put_u8(stop_sound_controls.into());

        if let Some(sound_category) = &self.sound_category {
            sound_category.serialize(buf)?;
        }
        if let Some(sound_name) = &self.sound_name {
            sound_name.serialize(buf)?;
        }

        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        let stop_sound_controls = StopSoundControls(buf.get_u8());

        let sound_category = if stop_sound_controls.category() {
            Some(SoundCategory::deserialize(buf)?)
        } else {
            None
        };
        let sound_name = if stop_sound_controls.name() {
            Some(NamespacedKey::deserialize(buf)?)
        } else {
            None
        };

        Ok(Self {
            sound_category,
            sound_name,
        })
    }
}

#[bitfield(u8)]
#[derive(ProtocolSerializer)]
pub struct StopSoundControls {
    pub category: bool,
    pub name: bool,
    #[bits(6)]
    __: u8,
}
