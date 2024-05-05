use crab_nbt::NbtTag;
use derive_more::{From, Into};
use std::fmt::Debug;

use crab_protocol::bounded::Bounded;
use crab_protocol::protocol_serializer::ProtocolSerializer;
use crab_protocol_macros::ProtocolSerializer;

use crate::buf::{ReadByteBuf, WriteByteBuf};
use crate::error::ProtocolError;

const MAX_JSON_TEXT_COMPONENT_LENGTH: usize = i16::MAX as usize;

#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, From, Into)]
pub struct JsonTextComponent(pub String);

impl ProtocolSerializer for JsonTextComponent {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        Bounded::check(&self.0, MAX_JSON_TEXT_COMPONENT_LENGTH)?;
        buf.put_string(&self.0);
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        let value = buf.get_string()?;
        Bounded::check(&value, MAX_JSON_TEXT_COMPONENT_LENGTH)?;
        Ok(value.into())
    }
}

#[derive(ProtocolSerializer, Clone, PartialEq, Debug, From, Into)]
pub struct TextComponent {
    pub tag: NbtTag,
}
