use crab_nbt::Nbt;

use crab_macros::ProtocolSerializer;

use crate::error::ProtocolError;

#[derive(Clone, PartialEq, Debug)]
pub struct ItemStack {
    pub id: i32,
    pub amount: u8,
    pub nbt: Nbt,
}

impl ItemStack {
    pub const fn new(id: i32, amount: u8, nbt: Nbt) -> Result<Self, ProtocolError> {
        Ok(Self { id, amount, nbt })
    }
}

impl ::crab_protocol::protocol_serializer::ProtocolSerializer for ItemStack {
    fn serialize(&self, mut buf: &mut impl ::crab_protocol::buf::WriteByteBuf) -> Result<(), ::crab_protocol::error::ProtocolError> {
        ::crab_protocol::protocol_serializer::ProtocolSerializer::serialize(&self.id, &mut buf)?;
        ::crab_protocol::protocol_serializer::ProtocolSerializer::serialize(&self.amount, &mut buf)?;
        ::crab_protocol::protocol_serializer::ProtocolSerializer::serialize(&self.nbt, &mut buf)?;
        Ok(())
    }
    fn deserialize(buf: &mut impl ::crab_protocol::buf::ReadByteBuf) -> Result<Self, ::crab_protocol::error::ProtocolError> { Ok(Self {
        id: ::crab_protocol::protocol_serializer::ProtocolSerializer::deserialize(buf)?,
        amount: ::crab_protocol::protocol_serializer::ProtocolSerializer::deserialize(buf)?,
        nbt: ::crab_protocol::protocol_serializer::ProtocolSerializer::deserialize(buf)? })
    }
}