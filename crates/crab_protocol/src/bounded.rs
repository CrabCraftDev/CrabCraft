use bytes::Bytes;

use crab_protocol::error::ProtocolError;

use crate::types::bytes::EndBytes;

/// The `Bounded` trait defines a common interface for types that can be checked against a maximum length.
/// In Minecraft Protocol used only for strings, lists and byte arrays.
pub trait Bounded {
    fn get_len(&self) -> usize;

    fn check(&self, max_length: usize) -> Result<(), ProtocolError> {
        if self.get_len() > max_length {
            return Err(ProtocolError::InvalidBoundedLength(self.get_len(), max_length));
        }
        Ok(())
    }
}

impl Bounded for String {
    fn get_len(&self) -> usize {
        self.chars().count()
    }
}

impl Bounded for Bytes {
    fn get_len(&self) -> usize {
        self.len()
    }
}

impl Bounded for EndBytes {
    fn get_len(&self) -> usize {
        self.0.len()
    }
}

impl<T: Bounded> Bounded for Option<T> {
    fn get_len(&self) -> usize {
        match self {
            Some(value) => value.get_len(),
            None => 0,
        }
    }
}

impl<T> Bounded for Vec<T> {
    fn get_len(&self) -> usize {
        self.len()
    }
}
