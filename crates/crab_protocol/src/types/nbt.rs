use crate::buf::{ReadByteBuf, WriteByteBuf};
use crate::error::ProtocolError;
use crab_nbt::{Nbt, NbtTag};
use crab_protocol::protocol_type::ProtocolType;

impl ProtocolType for NbtTag {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        buf.put(self.serialize_data());
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        Ok(NbtTag::deserialize(buf)?)
    }
}

impl ProtocolType for Nbt {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        buf.put(self.write_unnamed());
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        Ok(Nbt::read_unnamed(buf)?)
    }
}
