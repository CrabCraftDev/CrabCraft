use bytes::{Buf, Bytes};
use derive_more::{From, Into};

use crab_protocol::protocol_type::ProtocolType;

use crate::buf::{ReadByteBuf, WriteByteBuf};
use crate::error::ProtocolError;

impl ProtocolType for Bytes {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        buf.put_byte_array(self.as_ref());
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        buf.get_byte_array()
    }
}

#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, From, Into)]
#[repr(transparent)]
pub struct FixedBytes<const N: usize>(pub Bytes);

impl<const N: usize> ProtocolType for FixedBytes<N> {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        if self.0.remaining() != N {
            return Err(ProtocolError::InvalidFixedBytesLength(N, self.0.remaining()));
        }
        buf.put_raw_bytes(self.0.as_ref());
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        Ok(buf.get_raw_bytes(N as i32).into())
    }
}

/// Bytes buffer which length is inferred from the length of the packet
/// This type can only be used last since it consumes the rest of the buffer
#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, From, Into)]
#[repr(transparent)]
pub struct LastBytes(pub Bytes);

impl ProtocolType for LastBytes {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        buf.put_raw_bytes(self.0.as_ref());

        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        Ok(buf.get_raw_bytes(buf.remaining() as i32).into())
    }
}
