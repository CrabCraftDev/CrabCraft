use strum::FromRepr;

use crab_macros::{Packet, ProtocolSerializer};

use crate::buf::{ReadByteBuf, WriteByteBuf};
use crate::error::ProtocolError;
use crate::protocol_serializer::ProtocolSerializer;
use crate::protocol_state::ProtocolState::*;
use crate::types::number_format::NumberFormat;
use crab_text::text::Text;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "set_objective", id = 0x64, state = Play)]
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

impl ProtocolSerializer for UpdateObjectiveMode {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        match self {
            UpdateObjectiveMode::CreateScoreboard(content) | UpdateObjectiveMode::UpdateDisplayText(content) => {
                let mode = if let UpdateObjectiveMode::CreateScoreboard(_) = self {
                    0
                } else {
                    2
                };
                buf.put_i8(mode);
                content.serialize(buf)?;
            }
            UpdateObjectiveMode::RemoveScoreboard => {
                buf.put_i8(1);
            }
        }
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        let mode = buf.get_i8();
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

#[derive(Clone, PartialEq, Debug, ProtocolSerializer)]
pub struct ObjectiveContent {
    pub objective_value: Text,
    pub objective_type: ObjectiveType,
    pub number_format: Option<NumberFormat>,
}

#[derive(Debug, Copy, Clone, PartialEq, ProtocolSerializer, FromRepr)]
pub enum ObjectiveType {
    Integer,
    Hearts,
}
