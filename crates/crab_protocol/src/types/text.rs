use crate::buf::{ReadByteBuf, WriteByteBuf};
use crate::error::ProtocolError;
use crab_nbt::serde::de::from_bytes_unnamed;
use crab_protocol::protocol_serializer::ProtocolSerializer;
use crab_text::formats::TextFormat;
use crab_text::formats::nbt::NbtTextFormat;
use crab_text::text::Text;

impl ProtocolSerializer for Text {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        buf.put(NbtTextFormat.write(self)?);
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        Ok(from_bytes_unnamed(buf)?)
    }
}
