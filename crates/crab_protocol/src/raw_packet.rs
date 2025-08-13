use std::fmt::Debug;

use bytes::{BufMut, Bytes, BytesMut};

use crate::{buf::WriteByteBuf, error::ProtocolError, packet::Packet};
use crate::{packet, protocol_state::ProtocolState};

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
    pub fn decode<P: Packet>(&self, state: ProtocolState) -> Option<P> {
        if self.id != packet::get_id_for_state::<P>(state)? {
            return None;
        }

        let mut buffer = self.data.as_ref();
        if let Ok(pkt) = P::deserialize(&mut buffer)
            && buffer.is_empty()
        {
            return Some(pkt);
        }

        None
    }

    pub fn to_bytes(&self) -> Bytes {
        let mut data = BytesMut::new();
        data.put_var_int(self.id);
        data.put(self.data.clone());
        data.freeze()
    }

    pub fn try_from_packet<P: Packet>(value: P, state: ProtocolState) -> Result<Self, ProtocolError> {
        let mut data = BytesMut::new();
        value.serialize(&mut data)?;
        let Some(id) = packet::get_id_for_state::<P>(state) else {
            return Err(ProtocolError::InvalidState(
                std::any::type_name::<Self>().to_string(),
                state,
            ));
        };

        Ok(RawPacket {
            data: data.freeze(),
            id,
        })
    }
}
