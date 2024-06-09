use std::fmt::Debug;

use bytes::{Bytes, BytesMut};

use crate::buf::WriteByteBuf;
use crate::error::ProtocolError;
use crate::protocol::{PacketDirection, ProtocolState};
use crate::protocol_serializer::ProtocolSerializer;

pub trait Packet: Sized + Debug + ProtocolSerializer {
    const SIDE: PacketDirection;
    const STATE_IDS: &'static [(ProtocolState, i32)];

    fn get_id_for_state(check_state: ProtocolState) -> Option<i32> {
        Self::STATE_IDS
            .iter()
            .find_map(|(state, id)| if *state == check_state { Some(*id) } else { None })
    }

    fn serialize_packet(&self, state: ProtocolState) -> Result<Bytes, ProtocolError> {
        let mut data = BytesMut::new();
        if let Some(id) = Self::get_id_for_state(state) {
            data.put_var_int(id);
            self.serialize(&mut data)?;
            Ok(data.freeze())
        } else {
            Err(ProtocolError::InvalidState(
                std::any::type_name::<Self>().to_string(),
                state,
            ))
        }
    }
}
