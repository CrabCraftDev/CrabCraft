use crate::buf::{ReadByteBuf, WriteByteBuf};
use crate::error::ProtocolError;
use crab_protocol::protocol_type::ProtocolType;
use derive_more::{From, Into};

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, From, Into)]
#[repr(transparent)]
pub struct VarLong(pub i64);

impl ProtocolType for VarLong {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        buf.put_var_long((*self).into());
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        Ok(buf.get_var_long()?.into())
    }
}
