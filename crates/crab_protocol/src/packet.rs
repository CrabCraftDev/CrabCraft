use std::fmt::Debug;

use bytes::{Bytes, BytesMut};

use crab_protocol::buf::WriteByteBuf;
use crab_protocol::protocol::{PacketDirection, ProtocolState};

use crate::buf::ReadByteBuf;
use crate::error::ProtocolError;

pub trait Packet: Sized + Debug {
    const ID: i32;
    const SIDE: PacketDirection;
    const STATE: ProtocolState;

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError>;
    fn serialize_content(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError>;

    fn serialize(&self) -> Result<Bytes, ProtocolError> {
        let mut data = BytesMut::new();
        data.put_var_int(Self::ID);
        self.serialize_content(&mut data)?;
        Ok(data.freeze())
    }
}
