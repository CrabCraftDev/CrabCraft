use crate::error::ProtocolError;
use bytes::Buf;
use bytes::BufMut;
use uuid::Uuid;

use super::CONTINUE_BIT;
use super::SEGMENT_BITS;

pub trait WriteByteBuf: BufMut + Sized {
    fn put_byte(&mut self, value: i8) {
        self.put_i8(value)
    }

    fn put_unsigned_byte(&mut self, value: u8) {
        self.put_u8(value)
    }

    fn put_var_int(&mut self, mut value: i32) {
        loop {
            if (value & !SEGMENT_BITS) == 0 {
                self.put_byte(value as i8);
                return;
            }

            self.put_byte(((value & SEGMENT_BITS) | CONTINUE_BIT) as i8);

            value >>= 7;
        }
    }

    fn put_var_long(&mut self, mut value: i64) {
        loop {
            if (value & !(SEGMENT_BITS as i64)) == 0 {
                self.put_byte(value as i8);
                return;
            }

            self.put_byte(((value & SEGMENT_BITS as i64) | CONTINUE_BIT as i64) as i8);

            value >>= 7;
        }
    }

    fn put_string(&mut self, data: String) -> Result<(), ProtocolError>
    where
        Self: Sized,
    {
        let bytes = data.as_bytes();
        self.put_byte_array(bytes)
    }

    fn put_boolean(&mut self, value: bool) {
        self.put_byte(if value { 1 } else { 0 });
    }

    fn put_raw_bytes(&mut self, value: impl Buf) {
        self.put(value)
    }

    fn put_byte_array(&mut self, value: impl Buf) -> Result<(), ProtocolError> {
        self.put_var_int(value.remaining().try_into()?);
        self.put(value);
        Ok(())
    }

    // fn put_some_array<T: ProtocolType>(&mut self, values: Vec<T>) -> Result<(), ProtocolError> {
    //     self.put_var_int(values.len().try_into()?);
    //
    //     for val in values {
    //         val.serialize(self)?;
    //     }
    //     Ok(())
    // }
    //
    // fn put_optional<T: ProtocolType>(&mut self, value: Option<T>) -> Result<(), ProtocolError> {
    //     match value {
    //         Some(value) => {
    //             self.put_boolean(true);
    //             value.serialize(self)?;
    //         }
    //         None => self.put_boolean(false),
    //     }
    //     Ok(())
    // }
    //
    // fn put_hash_map<K: ProtocolType + Eq + Hash, V: ProtocolType>(
    //     &mut self,
    //     value: HashMap<K, V>,
    // ) -> Result<(), ProtocolError> {
    //     self.put_var_int(value.len().try_into()?);
    //     for (key, value) in value {
    //         key.serialize(self)?;
    //         value.serialize(self)?;
    //     }
    //     Ok(())
    // }

    fn put_uuid(&mut self, value: Uuid) {
        self.put_u128(value.as_u128());
    }
}

impl<T: BufMut> WriteByteBuf for T {}
