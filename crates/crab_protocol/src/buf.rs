use std::io::ErrorKind;

use bytes::Bytes;
pub use read::*;
use tokio::io::AsyncReadExt;
pub use write::*;

use crate::error::ProtocolError;

pub(crate) mod read;
pub(crate) mod write;

pub const CONTINUE_BIT: i32 = 0x80;
pub const SEGMENT_BITS: i32 = 0x7F;

pub async fn packet_from_stream(stream: &mut (impl AsyncReadExt + Unpin)) -> Result<Option<Bytes>, ProtocolError> {
    let packet_len = get_var_num!(
        i32,
        match stream.read_i8().await {
            Err(ref err) if err.kind() == ErrorKind::UnexpectedEof => return Ok(None),
            other => other,
        }?
    )?;
    let mut buffer = vec![0; packet_len as usize];
    match stream.read_exact(&mut buffer).await? {
        0 => Ok(None),
        _ => Ok(Some(buffer.into())),
    }
}
