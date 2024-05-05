use std::fmt::Debug;

use bytes::{Bytes, BytesMut};

use crab_protocol::buf::WriteByteBuf;
use crab_protocol::protocol::{PacketDirection, ProtocolState};

use crate::error::ProtocolError;
use crate::protocol_serializer::ProtocolSerializer;

pub trait Packet: Sized + Debug + ProtocolSerializer {
    const ID: i32;
    const SIDE: PacketDirection;
    const STATE: ProtocolState;

    fn serialize_packet(&self) -> Result<Bytes, ProtocolError> {
        let mut data = BytesMut::new();
        data.put_var_int(Self::ID);
        self.serialize(&mut data)?;
        Ok(data.freeze())
    }
}
