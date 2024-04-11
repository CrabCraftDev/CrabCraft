use crate::buf::{ReadByteBuf, WriteByteBuf};
use crate::error::ProtocolError;
use crab_protocol::protocol_type::ProtocolType;

impl ProtocolType for i8 {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        buf.put_i8(*self);
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<i8, ProtocolError> {
        Ok(buf.get_i8())
    }
}

impl ProtocolType for i16 {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        buf.put_i16(*self);
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<i16, ProtocolError> {
        Ok(buf.get_i16())
    }
}

impl ProtocolType for i32 {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        buf.put_i32(*self);
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<i32, ProtocolError> {
        Ok(buf.get_i32())
    }
}

impl ProtocolType for i64 {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        buf.put_i64(*self);
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<i64, ProtocolError> {
        Ok(buf.get_i64())
    }
}

impl ProtocolType for u8 {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        buf.put_u8(*self);
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<u8, ProtocolError> {
        Ok(buf.get_u8())
    }
}

impl ProtocolType for u16 {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        buf.put_u16(*self);
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<u16, ProtocolError> {
        Ok(buf.get_u16())
    }
}

impl ProtocolType for f32 {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        buf.put_f32(*self);
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<f32, ProtocolError> {
        Ok(buf.get_f32())
    }
}

impl ProtocolType for f64 {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        buf.put_f64(*self);
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<f64, ProtocolError> {
        Ok(buf.get_f64())
    }
}

impl ProtocolType for bool {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        buf.put_boolean(*self);
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        Ok(buf.get_boolean())
    }
}
