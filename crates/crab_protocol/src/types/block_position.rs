use crab_protocol::protocol_serializer::ProtocolSerializer;

use crate::buf::{ReadByteBuf, WriteByteBuf};
use crate::error::ProtocolError;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct BlockPosition {
    pub x: i32,
    pub y: i16,
    pub z: i32,
}

const X_BITS: u32 = 26;
const Y_BITS: u32 = 12;
const Z_BITS: u32 = 26;

impl BlockPosition {
    pub fn new(x: i32, y: i16, z: i32) -> Result<Self, ProtocolError> {
        fits_in_bits(x, X_BITS, "x")?;
        fits_in_bits(y as i32, Y_BITS, "y")?;
        fits_in_bits(z, Z_BITS, "z")?;

        Ok(Self { x, y, z })
    }
}

/// Position protocol type is a 64-bit value split into 3 signed integer parts
/// X as a 26-bit integer, followed by z as a 26-bit integer, followed by y as a 12-bit integer (all signed, two's complement).
impl ProtocolSerializer for BlockPosition {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        let x = (self.x as i64 & 0x3FFFFFF) << (64 - X_BITS);
        let z = (self.z as i64 & 0x3FFFFFF) << (64 - X_BITS - Z_BITS);
        let y = (self.y as i64) & 0xFFF;

        buf.put_i64(x | z | y);
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        let val = buf.get_i64();
        let x: i32 = (val >> (64 - X_BITS)) as i32;
        let z: i32 = (val << X_BITS >> (64 - Z_BITS)) as i32;
        let y: i16 = (val << (X_BITS + Z_BITS) >> (64 - Y_BITS)) as i16;
        Self::new(x, y, z)
    }
}

fn fits_in_bits(number: i32, amount_of_bits: u32, coordinate_name: &str) -> Result<(), ProtocolError> {
    let max_value = (1 << (amount_of_bits - 1)) - 1;
    let min_value = -(1 << (amount_of_bits - 1));

    if number <= max_value && number >= min_value {
        return Ok(());
    };

    Err(ProtocolError::InvalidBlockPos(
        coordinate_name.to_string(),
        number,
        amount_of_bits,
    ))
}
