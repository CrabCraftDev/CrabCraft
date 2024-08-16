pub use read::*;
pub use write::*;

pub(crate) mod read;
pub(crate) mod write;

pub const CONTINUE_BIT: i32 = 0x80;
pub const SEGMENT_BITS: i32 = 0x7F;
