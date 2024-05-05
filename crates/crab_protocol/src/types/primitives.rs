use crab_protocol::protocol_serializer::ProtocolSerializer;

use crate::buf::{ReadByteBuf, WriteByteBuf};
use crate::error::ProtocolError;

impl ProtocolSerializer for i8 {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        buf.put_i8(*self);
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<i8, ProtocolError> {
        Ok(buf.get_i8())
    }
}

impl ProtocolSerializer for i16 {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        buf.put_i16(*self);
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<i16, ProtocolError> {
        Ok(buf.get_i16())
    }
}

impl ProtocolSerializer for i32 {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        buf.put_i32(*self);
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<i32, ProtocolError> {
        Ok(buf.get_i32())
    }
}

impl ProtocolSerializer for i64 {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        buf.put_i64(*self);
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<i64, ProtocolError> {
        Ok(buf.get_i64())
    }
}

impl ProtocolSerializer for u8 {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        buf.put_u8(*self);
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<u8, ProtocolError> {
        Ok(buf.get_u8())
    }
}

impl ProtocolSerializer for u16 {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        buf.put_u16(*self);
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<u16, ProtocolError> {
        Ok(buf.get_u16())
    }
}

impl ProtocolSerializer for f32 {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        buf.put_f32(*self);
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<f32, ProtocolError> {
        Ok(buf.get_f32())
    }
}

impl ProtocolSerializer for f64 {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        buf.put_f64(*self);
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<f64, ProtocolError> {
        Ok(buf.get_f64())
    }
}

impl ProtocolSerializer for bool {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        buf.put_bool(*self);
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        Ok(buf.get_bool())
    }
}
