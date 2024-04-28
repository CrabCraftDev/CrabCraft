use strum::FromRepr;

use crab_protocol::buf::{ReadByteBuf, WriteByteBuf};
use crab_protocol::error::ProtocolError;
use crab_protocol::protocol_type::ProtocolType;
use crab_protocol::types::number_format::NumberFormat;
use crab_protocol_macros::{Packet, ProtocolType};

use crate::protocol::ProtocolState::*;
use crate::types::text::TextComponent;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x5C, state = Play)]
pub struct UpdateObjectivesS2CPacket {
    pub objective_name: String,
    pub mode: UpdateObjectiveMode,
}

#[derive(Debug, Clone)]
pub enum UpdateObjectiveMode {
    CreateScoreboard(ObjectiveContent),
    RemoveScoreboard,
    UpdateDisplayText(ObjectiveContent),
}

impl ProtocolType for UpdateObjectiveMode {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        match self {
            UpdateObjectiveMode::CreateScoreboard(content) | UpdateObjectiveMode::UpdateDisplayText(content) => {
                let mode = if let UpdateObjectiveMode::CreateScoreboard(_) = self {
                    0
                } else {
                    2
                };
                buf.put_byte(mode);
                content.serialize(buf)?;
            }
            UpdateObjectiveMode::RemoveScoreboard => {
                buf.put_byte(1);
            }
        }
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        let mode = buf.get_byte();
        match mode {
            0 | 2 => {
                let content = ObjectiveContent::deserialize(buf)?;
                if mode == 0 {
                    Ok(UpdateObjectiveMode::CreateScoreboard(content))
                } else {
                    Ok(UpdateObjectiveMode::UpdateDisplayText(content))
                }
            }
            1 => Ok(UpdateObjectiveMode::RemoveScoreboard),
            _ => Err(ProtocolError::EnumOutOfBounds(
                "UpdateObjectiveMode".to_string(),
                mode as usize,
            )),
        }
    }
}

#[derive(Clone, PartialEq, Debug, ProtocolType)]
pub struct ObjectiveContent {
    pub objective_value: TextComponent,
    pub objective_type: ObjectiveType,
    pub number_format: Option<NumberFormat>,
}

#[derive(Debug, Copy, Clone, PartialEq, ProtocolType, FromRepr)]
pub enum ObjectiveType {
    Integer,
    Hearts,
}
