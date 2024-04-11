use crab_protocol::buf::{ReadByteBuf, WriteByteBuf};
use crab_protocol::error::ProtocolError;
use crab_protocol::protocol_type::ProtocolType;
use derive_more::{From, Into};

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, From, Into)]
pub struct Angle(pub u8);

impl Angle {
    pub fn from_degrees(degrees: f32) -> Self {
        Angle((degrees * (256.0 / 360.0)) as u8)
    }

    pub fn to_degrees(&self) -> f32 {
        self.0 as f32 / 256.0 * 360.0
    }
}

impl ProtocolType for Angle {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        buf.put_unsigned_byte((*self).into());
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        Ok(Self::from_degrees(buf.get_unsigned_byte() as f32))
    }
}
