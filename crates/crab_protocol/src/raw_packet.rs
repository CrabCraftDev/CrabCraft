use std::fmt::Debug;

use bytes::{BufMut, Bytes, BytesMut};

use crate::{buf::WriteByteBuf, error::ProtocolError, packet::Packet};

#[derive(Debug, Clone)]
pub struct RawPacket {
    pub id: i32,
    pub data: Bytes,
}

impl RawPacket {
    pub const fn new(id: i32, data: Bytes) -> Self {
        RawPacket { id, data }
    }

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

    pub fn to_bytes(&self) -> Bytes {
        let mut data = BytesMut::new();
        data.put_var_int(self.id);
        data.put(self.data.clone());
        data.freeze()
    }

    pub fn from_packet<T: Packet>(value: T) -> Result<Self, ProtocolError> {
        let mut data = BytesMut::new();
        value.serialize(&mut data)?;
        Ok(RawPacket {
            data: data.freeze(),
            id: T::ID,
        })
    }
}
