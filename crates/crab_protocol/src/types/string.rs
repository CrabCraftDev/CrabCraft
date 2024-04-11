use crab_protocol::bounded::Bounded;
use crab_protocol::protocol_type::ProtocolType;

use crate::buf::{ReadByteBuf, WriteByteBuf};
use crate::error::ProtocolError;

const MAX_STRING_LENGTH: usize = 32767;

impl ProtocolType for String {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        Bounded::check(self, MAX_STRING_LENGTH)?;
        buf.put_byte_array(self.as_bytes());
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        let value = buf.get_string()?;
        Bounded::check(&value, MAX_STRING_LENGTH)?;
        Ok(value)
    }
}
