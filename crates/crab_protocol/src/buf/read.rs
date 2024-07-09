use std::str::from_utf8;

use bytes::{Buf, Bytes};

use crate::error::ProtocolError;
use crate::protocol_serializer::ProtocolSerializer;

use super::CONTINUE_BIT;
use super::SEGMENT_BITS;

/// Function-like macro for reading varints and varlongs from various sources.  
///  
/// # Arguments
/// - `i32` or `i64`: For varint and varlong respectively  
/// - Expression that returns the next byte when called each time  
///  
/// # Returns  
/// - `Result<i32/i64, ProtocolError>`
macro_rules! get_var_num {
    ($num_type:ty, $getter:expr) => {{
        let mut value: $num_type = 0;
        let mut position: i8 = 0;
        loop {
            let current_byte = $getter;

            value |= (current_byte as $num_type & SEGMENT_BITS as $num_type) << position;
            if (current_byte as $num_type & CONTINUE_BIT as $num_type) == 0 {
                break Result::<$num_type, ProtocolError>::Ok(value);
            }

            position += 7;
            if position >= <$num_type>::BITS as i8 {
                break Result::<$num_type, ProtocolError>::Err(ProtocolError::TooBigVarInt(32, position + 1).into());
            }
        }
    }};
}

pub(crate) use get_var_num;

pub trait ReadByteBuf: Buf + Sized {
    fn get_var_int(&mut self) -> Result<i32, ProtocolError> {
        get_var_num!(i32, self.get_i8())
    }

    fn get_var_long(&mut self) -> Result<i64, ProtocolError> {
        get_var_num!(i64, self.get_i8())
    }

    fn get_string(&mut self) -> Result<String, ProtocolError> {
        let bytes = self.get_byte_array()?;
        let str = from_utf8(bytes.chunk())?;
        Ok(str.to_string())
    }

    fn get_bool(&mut self) -> bool {
        self.get_i8() != 0
    }

    fn get_raw_bytes(&mut self, length: i32) -> Bytes {
        self.copy_to_bytes(length as usize)
    }

    fn get_byte_array(&mut self) -> Result<Bytes, ProtocolError> {
        let length = self.get_var_int()?;
        let bytes = self.get_raw_bytes(length);
        Ok(bytes)
    }

    fn get_array<T: ProtocolSerializer>(&mut self) -> Result<Vec<T>, ProtocolError> {
        let length = self.get_var_int()?;
        let mut items = Vec::with_capacity(length as usize);

        for _i in 0..length {
            items.push(T::deserialize(self)?);
        }

        Ok(items)
    }

    fn get_optional<T: ProtocolSerializer>(&mut self) -> Result<Option<T>, ProtocolError> {
        match self.get_bool() {
            true => Ok(Some(T::deserialize(self)?)),
            false => Ok(None),
        }
    }
}

impl<T: Buf> ReadByteBuf for T {}
