use std::fmt::Debug;

use bytes::Bytes;

use crate::packet::Packet;

#[derive(Debug, Clone)]
pub struct RawPacket {
    pub id: i32,
    pub data: Bytes,
}

impl RawPacket {
    pub const fn new(id: i32, data: Bytes) -> Self {
        RawPacket { id, data }
    }
}

impl RawPacket {
    /// Try to decode the raw packet into a packet
    /// Returns None if the packet can't be converted
    pub fn decode<P: Packet>(&self) -> Option<P> {
        if self.id != P::ID {
            return None;
        }

        let mut buffer = self.data.as_ref();
        if let Ok(pkt) = P::deserialize(&mut buffer) {
            if buffer.is_empty() {
                return Some(pkt);
            }
        }

        None
    }
}
