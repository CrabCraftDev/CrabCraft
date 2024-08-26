use bitfield_struct::bitfield;
use strum::FromRepr;
use uuid::Uuid;

use crab_macros::{Packet, ProtocolSerializer};

use crate::buf::{ReadByteBuf, WriteByteBuf};
use crate::error::ProtocolError;
use crate::protocol_serializer::ProtocolSerializer;
use crate::protocol_state::ProtocolState::*;
use crate::types::text::TextComponent;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x0A, state = Play)]
pub struct BossBarS2CPacket {
    pub boss_bar_id: Uuid,
    pub method: BossBarMethod,
}

#[derive(Debug, Clone)]
pub enum BossBarMethod {
    Add {
        title: TextComponent,
        health: f32,
        color: BossBarColor,
        division: BossBarDivision,
        flags: BossBarFlags,
    },
    Remove,
    UpdateHealth(f32),
    UpdateTitle(TextComponent),
    UpdateStyle {
        color: BossBarColor,
        division: BossBarDivision,
    },
    UpdateFlags(BossBarFlags),
}

impl ProtocolSerializer for BossBarMethod {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        match self {
            BossBarMethod::Add {
                title,
                health,
                color,
                division,
                flags,
            } => {
                buf.put_var_int(0);
                title.serialize(buf)?;
                buf.put_f32(*health);
                color.serialize(buf)?;
                division.serialize(buf)?;
                flags.serialize(buf)?;
            }
            BossBarMethod::Remove => {
                buf.put_var_int(1);
            }
            BossBarMethod::UpdateHealth(health) => {
                buf.put_var_int(2);
                buf.put_f32(*health);
            }
            BossBarMethod::UpdateTitle(title) => {
                buf.put_var_int(3);
                title.serialize(buf)?;
            }
            BossBarMethod::UpdateStyle { color, division } => {
                buf.put_var_int(4);
                color.serialize(buf)?;
                division.serialize(buf)?;
            }
            BossBarMethod::UpdateFlags(flags) => {
                buf.put_var_int(5);
                flags.serialize(buf)?;
            }
        }
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        let method = buf.get_var_int()?;
        match method {
            0 => Ok(BossBarMethod::Add {
                title: ProtocolSerializer::deserialize(buf)?,
                health: buf.get_f32(),
                color: ProtocolSerializer::deserialize(buf)?,
                division: ProtocolSerializer::deserialize(buf)?,
                flags: ProtocolSerializer::deserialize(buf)?,
            }),
            1 => Ok(BossBarMethod::Remove),
            2 => Ok(BossBarMethod::UpdateHealth(buf.get_f32())),
            3 => Ok(BossBarMethod::UpdateTitle(ProtocolSerializer::deserialize(buf)?)),
            4 => Ok(BossBarMethod::UpdateStyle {
                color: ProtocolSerializer::deserialize(buf)?,
                division: ProtocolSerializer::deserialize(buf)?,
            }),
            5 => Ok(BossBarMethod::UpdateFlags(ProtocolSerializer::deserialize(buf)?)),
            _ => Err(ProtocolError::EnumOutOfBounds(
                "UpdateObjectiveMode".to_string(),
                method as usize,
            )),
        }
    }
}

#[bitfield(u8)]
#[derive(ProtocolSerializer)]
pub struct BossBarFlags {
    pub should_darken_sky: bool,
    pub is_dragon_bar: bool,
    pub create_fog: bool,
    #[bits(5)]
    __: u8,
}

#[derive(Debug, Copy, Clone, PartialEq, ProtocolSerializer, FromRepr)]
pub enum BossBarColor {
    Pink,
    Blue,
    Red,
    Green,
    Yellow,
    Purple,
    White,
}

#[derive(Debug, Copy, Clone, PartialEq, ProtocolSerializer, FromRepr)]
pub enum BossBarDivision {
    NoDivision,
    Notched6,
    Notched10,
    Notched12,
    Notched20,
}
