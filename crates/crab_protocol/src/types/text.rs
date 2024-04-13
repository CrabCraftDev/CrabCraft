use crab_nbt::NbtTag;
use derive_more::{From, Into};

use crab_protocol::bounded::Bounded;
use crab_protocol::protocol_type::ProtocolType;
use crab_protocol_macros::ProtocolType;

use crate::buf::{ReadByteBuf, WriteByteBuf};
use crate::error::ProtocolError;

const MAX_JSON_TEXT_COMPONENT_LENGTH: usize = 32767;

#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, From, Into)]
pub struct JsonTextComponent(pub String);

impl ProtocolType for JsonTextComponent {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        Bounded::check(&self.0, MAX_JSON_TEXT_COMPONENT_LENGTH)?;
        buf.put_string(self.0.to_string());
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        let value = buf.get_string()?;
        Bounded::check(&value, MAX_JSON_TEXT_COMPONENT_LENGTH)?;
        Ok(value.into())
    }
}

#[derive(ProtocolType, Clone, PartialEq, Debug, From, Into)]
pub struct TextComponent {
    pub tag: NbtTag,
}
