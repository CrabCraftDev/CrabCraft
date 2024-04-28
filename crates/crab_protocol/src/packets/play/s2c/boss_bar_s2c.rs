use bitfield_struct::bitfield;
use strum::FromRepr;
use uuid::Uuid;

use crab_protocol::buf::{ReadByteBuf, WriteByteBuf};
use crab_protocol::error::ProtocolError;
use crab_protocol::protocol_type::ProtocolType;
use crab_protocol_macros::{Packet, ProtocolType};

use crate::protocol::ProtocolState::*;
use crate::types::text::TextComponent;

#[derive(Packet, Debug, Clone)]
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

impl ProtocolType for BossBarMethod {
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
                title: ProtocolType::deserialize(buf)?,
                health: buf.get_f32(),
                color: ProtocolType::deserialize(buf)?,
                division: ProtocolType::deserialize(buf)?,
                flags: ProtocolType::deserialize(buf)?,
            }),
            1 => Ok(BossBarMethod::Remove),
            2 => Ok(BossBarMethod::UpdateHealth(buf.get_f32())),
            3 => Ok(BossBarMethod::UpdateTitle(ProtocolType::deserialize(buf)?)),
            4 => Ok(BossBarMethod::UpdateStyle {
                color: ProtocolType::deserialize(buf)?,
                division: ProtocolType::deserialize(buf)?,
            }),
            5 => Ok(BossBarMethod::UpdateFlags(ProtocolType::deserialize(buf)?)),
            _ => Err(ProtocolError::EnumOutOfBounds(
                "UpdateObjectiveMode".to_string(),
                method as usize,
            )),
        }
    }
}

#[bitfield(u8)]
pub struct BossBarFlags {
    pub should_darken_sky: bool,
    pub is_dragon_bar: bool,
    pub create_fog: bool,
    #[bits(5)]
    __: u8,
}

impl ProtocolType for BossBarFlags {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        buf.put_i8(self.0 as i8);
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        Ok(Self(buf.get_i8() as u8))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, ProtocolType, FromRepr)]
pub enum BossBarColor {
    Pink,
    Blue,
    Red,
    Green,
    Yellow,
    Purple,
    White,
}

#[derive(Debug, Copy, Clone, PartialEq, ProtocolType, FromRepr)]
pub enum BossBarDivision {
    NoDivision,
    Notched6,
    Notched10,
    Notched12,
    Notched20,
}
