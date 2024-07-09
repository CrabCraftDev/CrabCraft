use bytes::{Bytes, BytesMut};

pub use read::*;
pub use write::*;

pub(crate) mod read;
pub(crate) mod write;

pub const CONTINUE_BIT: i32 = 0x80;
pub const SEGMENT_BITS: i32 = 0x7F;

pub fn to_var_int(value: i32) -> Bytes {
    let mut bytes = BytesMut::new();
    bytes.put_var_int(value);
    bytes.into()
}
