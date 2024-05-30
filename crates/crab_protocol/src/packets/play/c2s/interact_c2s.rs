use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::buf::{ReadByteBuf, WriteByteBuf};
use crate::error::ProtocolError;
use crate::protocol::ProtocolState::*;
use crate::protocol_serializer::ProtocolSerializer;
use crate::types::hand::Hand;
use crate::types::position::Position;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x16, state = Play)]
pub struct InteractC2SPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub entity_id: i32,
    pub action: InteractActionType,
    pub is_sneaking: bool,
}

#[derive(Clone, Copy, Debug)]
pub enum InteractActionType {
    Interact { hand: Hand },
    Attack,
    InteractAt { location: Position, hand: Hand },
}

impl ProtocolSerializer for InteractActionType {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        match self {
            InteractActionType::Interact { hand } => {
                buf.put_var_int(0);
                hand.serialize(buf)?;
            }
            InteractActionType::Attack => {
                buf.put_var_int(1);
            }
            InteractActionType::InteractAt { location, hand } => {
                buf.put_var_int(2);
                location.serialize(buf)?;
                hand.serialize(buf)?;
            }
        }
        Ok(())
    }
    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        let action_id = buf.get_var_int()?;
        match action_id {
            0 => {
                let hand = Hand::deserialize(buf)?;
                Ok(InteractActionType::Interact { hand })
            }
            1 => Ok(InteractActionType::Attack),
            2 => {
                let location = Position::deserialize(buf)?;
                let hand = Hand::deserialize(buf)?;
                Ok(InteractActionType::InteractAt { location, hand })
            }
            _ => Err(ProtocolError::EnumOutOfBounds(
                "InteractActionType".to_string(),
                action_id as usize,
            )),
        }
    }
}
