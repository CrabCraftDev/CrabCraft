use crab_macros::{Packet, ProtocolSerializer};

use crate::buf::{ReadByteBuf, WriteByteBuf};
use crate::error::ProtocolError;
use crate::protocol_serializer::ProtocolSerializer;
use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x5C, state = Play)]
pub struct DisplayObjectiveS2CPacket {
    pub objective_position: ObjectivePosition,
    pub score_name: String,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ObjectivePosition {
    List,
    Sidebar,
    BelowName,
    TeamSpecificSidebar(i32),
}

impl ProtocolSerializer for ObjectivePosition {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        buf.put_var_int(match self {
            ObjectivePosition::List => 0,
            ObjectivePosition::Sidebar => 1,
            ObjectivePosition::BelowName => 2,
            ObjectivePosition::TeamSpecificSidebar(id) => 3 + *id,
        });
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        let value = buf.get_var_int()?;
        match value {
            0 => Ok(ObjectivePosition::List),
            1 => Ok(ObjectivePosition::Sidebar),
            2 => Ok(ObjectivePosition::BelowName),
            3..=18 => Ok(ObjectivePosition::TeamSpecificSidebar(value - 3)),
            _ => Err(ProtocolError::EnumOutOfBounds(
                "ObjectivePosition".to_string(),
                value as usize,
            )),
        }
    }
}
