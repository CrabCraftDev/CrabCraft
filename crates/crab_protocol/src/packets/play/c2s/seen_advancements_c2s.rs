use crab_macros::{Packet, ProtocolSerializer};

use crate::buf::{ReadByteBuf, WriteByteBuf};
use crate::error::ProtocolError;
use crate::protocol_serializer::ProtocolSerializer;
use crate::protocol_state::ProtocolState::*;
use crate::types::namespaced_key::NamespacedKey;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "seen_advancements", id = 0x2E, state = Play)]
pub struct SeenAdvancementsC2SPacket {
    pub action: SeenAdvancementsAction,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SeenAdvancementsAction {
    OpenedTab(NamespacedKey),
    ClosedScreen,
}

impl ProtocolSerializer for SeenAdvancementsAction {
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
            0 => Ok(SeenAdvancementsAction::OpenedTab(ProtocolSerializer::deserialize(buf)?)),
            1 => Ok(SeenAdvancementsAction::ClosedScreen),
            _ => Err(ProtocolError::EnumOutOfBounds(
                "UpdateObjectiveMode".to_string(),
                action as usize,
            )),
        }
    }
}
