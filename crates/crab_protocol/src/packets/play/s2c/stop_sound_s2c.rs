use bitfield_struct::bitfield;

use crab_protocol::buf::{ReadByteBuf, WriteByteBuf};
use crab_protocol::error::ProtocolError;
use crab_protocol::protocol::{PacketDirection, ProtocolState};
use crab_protocol::protocol_type::ProtocolType;

use crate::packet::Packet;
use crate::types::identifier::Identifier;
use crate::types::sound::SoundCategory;

#[derive(Debug, Clone)]
pub struct StopSoundS2CPacket {
    pub sound_category: Option<SoundCategory>,
    pub sound_name: Option<Identifier>,
}

impl Packet for StopSoundS2CPacket {
    const ID: i32 = 0x68;
    const SIDE: PacketDirection = PacketDirection::Clientbound;
    const STATE: ProtocolState = ProtocolState::Play;

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        let stop_sound_controls = StopSoundControls(buf.get_u8());

        let sound_category = if stop_sound_controls.category() {
            Some(SoundCategory::deserialize(buf)?)
        } else {
            None
        };
        let sound_name = if stop_sound_controls.name() {
            Some(Identifier::deserialize(buf)?)
        } else {
            None
        };

        Ok(Self {
            sound_category,
            sound_name,
        })
    }

    fn serialize_content(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
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
}

#[bitfield(u8)]
pub struct StopSoundControls {
    pub category: bool,
    pub name: bool,
    #[bits(6)]
    __: u8,
}
