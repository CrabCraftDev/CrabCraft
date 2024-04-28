use bitfield_struct::bitfield;
use strum::FromRepr;

use crab_protocol::buf::{ReadByteBuf, WriteByteBuf};
use crab_protocol::error::ProtocolError;
use crab_protocol::protocol_type::ProtocolType;
use crab_protocol_macros::{Packet, ProtocolType};

use crate::protocol::ProtocolState::*;
use crate::types::text::TextComponent;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x5E, state = Play)]
pub struct UpdateTeamsS2CPacket {
    pub team_name: String,
    pub method: UpdateTeamsMethod,
}

#[derive(Debug, Clone)]
pub enum UpdateTeamsMethod {
    Create {
        team_information: TeamInformation,
        players: Vec<String>,
    },
    Remove,
    Update {
        team_information: TeamInformation,
    },
    AddEntities {
        added_players_list: Vec<String>,
    },
    RemoveEntities {
        removed_players_list: Vec<String>,
    },
}

impl ProtocolType for UpdateTeamsMethod {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        match self {
            UpdateTeamsMethod::Create {
                team_information,
                players,
            } => {
                buf.put_u8(0);
                team_information.serialize(buf)?;
                players.serialize(buf)?;
            }
            UpdateTeamsMethod::Remove => {
                buf.put_u8(1);
            }
            UpdateTeamsMethod::Update { team_information } => {
                buf.put_u8(2);
                team_information.serialize(buf)?;
            }
            UpdateTeamsMethod::AddEntities { added_players_list } => {
                buf.put_u8(3);
                added_players_list.serialize(buf)?;
            }
            UpdateTeamsMethod::RemoveEntities { removed_players_list } => {
                buf.put_u8(4);
                removed_players_list.serialize(buf)?;
            }
        }
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        let mode = buf.get_byte();
        match mode {
            0 => Ok(UpdateTeamsMethod::Create {
                team_information: ProtocolType::deserialize(buf)?,
                players: ProtocolType::deserialize(buf)?,
            }),
            1 => Ok(UpdateTeamsMethod::Remove),
            2 => Ok(UpdateTeamsMethod::Update {
                team_information: ProtocolType::deserialize(buf)?,
            }),
            3 => Ok(UpdateTeamsMethod::AddEntities {
                added_players_list: ProtocolType::deserialize(buf)?,
            }),
            4 => Ok(UpdateTeamsMethod::RemoveEntities {
                removed_players_list: ProtocolType::deserialize(buf)?,
            }),
            _ => Err(ProtocolError::EnumOutOfBounds(
                "UpdateTeamsMode".to_string(),
                mode as usize,
            )),
        }
    }
}

#[derive(Clone, Debug, ProtocolType)]
pub struct TeamInformation {
    pub team_display_name: TextComponent,
    pub friendly_flags: FriendlyFlags,
    pub name_tag_visibility: String,
    pub collision_rule: String,
    pub name_color: TextFormatting,
    pub name_prefix: TextComponent,
    pub name_suffix: TextComponent,
}

#[bitfield(u8)]
pub struct FriendlyFlags {
    pub allow_friendly_fire: bool,
    pub can_see_invisible_entities_on_same_team: bool,
    #[bits(6)]
    __: u8,
}

impl ProtocolType for FriendlyFlags {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        buf.put_i8(self.0 as i8);
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        Ok(Self(buf.get_i8() as u8))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, ProtocolType, FromRepr)]
pub enum TextFormatting {
    Black,
    DarkBlue,
    DarkGreen,
    DarkCyan,
    DarkRed,
    Purple,
    Gold,
    Gray,
    DarkGray,
    Blue,
    BrightGreen,
    Cyan,
    Red,
    Pink,
    Yellow,
    White,
    Obfuscated,
    Bold,
    Strikethrough,
    Underline,
    Italic,
    Reset,
}
