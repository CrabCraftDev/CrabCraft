use crab_protocol::buf::{ReadByteBuf, WriteByteBuf};
use crab_protocol::error::ProtocolError;
use crab_protocol::protocol_type::ProtocolType;
use crab_protocol::types::identifier::Identifier;
use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x29, state = Play)]
pub struct SeenAdvancementsC2SPacket {
    pub action: SeenAdvancementsAction,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SeenAdvancementsAction {
    OpenedTab(Identifier),
    ClosedScreen,
}

impl ProtocolType for SeenAdvancementsAction {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        match self {
            SeenAdvancementsAction::OpenedTab(tab_id) => {
                buf.put_var_int(0);
                tab_id.serialize(buf)?;
            }
            SeenAdvancementsAction::ClosedScreen => {
                buf.put_var_int(1);
            }
        }
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        let action = buf.get_var_int()?;
        match action {
            0 => Ok(SeenAdvancementsAction::OpenedTab(ProtocolType::deserialize(buf)?)),
            1 => Ok(SeenAdvancementsAction::ClosedScreen),
            _ => Err(ProtocolError::EnumOutOfBounds(
                "UpdateObjectiveMode".to_string(),
                action as usize,
            )),
        }
    }
}
