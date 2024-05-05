use crab_nbt::{Nbt, NbtTag};

use crab_protocol::protocol_serializer::ProtocolSerializer;

use crate::buf::{ReadByteBuf, WriteByteBuf};
use crate::error::ProtocolError;

impl ProtocolSerializer for NbtTag {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        buf.put(self.serialize_data());
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        Ok(NbtTag::deserialize(buf)?)
    }
}

impl ProtocolSerializer for Nbt {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        buf.put(self.write_unnamed());
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        Ok(Nbt::read_unnamed(buf)?)
    }
}
