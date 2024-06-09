use strum::FromRepr;
use uuid::Uuid;

use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::buf::{ReadByteBuf, WriteByteBuf};
use crate::error::ProtocolError;
use crate::protocol_serializer::ProtocolSerializer;
use crate::protocol_state::ProtocolState::*;
use crate::types::bit_set::BitSet;
use crate::types::bytes::FixedBytes;
use crate::types::text::TextComponent;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x39, state = Play)]
pub struct PlayerChatMessageS2CPacket {
    pub sender: Uuid,
    #[protocol_attribute(protocol_type = VarInt)]
    pub index: i32,
    pub message_signature: Option<FixedBytes<256>>,
    #[protocol_attribute(bounded = 256)]
    pub message: String,
    pub timestamp: i64,
    pub salt: i64,
    #[protocol_attribute(bounded = 20)]
    pub previous_messages: Vec<PreviousMessage>,
    pub unsigned_content: Option<TextComponent>,
    pub filter_type: MessageFilterType,
    pub sender_name: Option<TextComponent>,
    pub target_name: Option<TextComponent>,
}

#[derive(Clone, Debug, PartialEq, FromRepr)]
pub enum MessageFilterType {
    PassThrough,
    FullyFiltered,
    PartiallyFiltered(BitSet),
}

impl ProtocolSerializer for MessageFilterType {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        match self {
            MessageFilterType::PassThrough => {
                buf.put_var_int(0);
            }
            MessageFilterType::FullyFiltered => {
                buf.put_var_int(1);
            }
            MessageFilterType::PartiallyFiltered(bit_set) => {
                buf.put_var_int(2);
                bit_set.serialize(buf)?;
            }
        }
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        let filter_type = buf.get_var_int()?;
        match filter_type {
            0 => Ok(MessageFilterType::PassThrough),
            1 => Ok(MessageFilterType::FullyFiltered),
            2 => {
                let bit_set = BitSet::deserialize(buf)?;
                Ok(MessageFilterType::PartiallyFiltered(bit_set))
            }
            _ => Err(ProtocolError::EnumOutOfBounds(
                "MessageFilterType".to_string(),
                filter_type as usize,
            )),
        }
    }
}

#[derive(Debug, ProtocolSerializer, Clone, PartialEq)]
pub struct PreviousMessage {
    #[protocol_attribute(protocol_type = VarInt)]
    pub message_id: i32,
    pub message_signature: MessageSignature,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MessageSignature {
    Signature(FixedBytes<256>),
    Id(i32),
}

impl ProtocolSerializer for MessageSignature {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        match self {
            MessageSignature::Signature(signature) => {
                signature.serialize(buf)?;
            }
            MessageSignature::Id(id) => {
                VarInt::from(id + 1).serialize(buf)?;
            }
        }
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        let message_id = buf.get_var_int()?;
        if message_id == 0 {
            let signature = FixedBytes::deserialize(buf)?;
            Ok(MessageSignature::Signature(signature))
        } else {
            let id = VarInt::deserialize(buf)?.0 - 1;
            Ok(MessageSignature::Id(id))
        }
    }
}
