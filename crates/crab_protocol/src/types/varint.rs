use bytes::{Bytes, BytesMut};
use derive_more::{From, Into};

use crab_protocol::protocol_serializer::ProtocolSerializer;

use crate::buf::{ReadByteBuf, WriteByteBuf};
use crate::error::ProtocolError;

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, From, Into)]
#[repr(transparent)]
pub struct VarInt(pub i32);

impl VarInt {
    pub fn as_bytes(&self) -> Bytes {
        let mut bytes = BytesMut::new();
        bytes.put_var_int(self.0);
        bytes.freeze()
    }
}

impl ProtocolSerializer for VarInt {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        buf.put_var_int((*self).into());
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        Ok(buf.get_var_int()?.into())
    }
}
