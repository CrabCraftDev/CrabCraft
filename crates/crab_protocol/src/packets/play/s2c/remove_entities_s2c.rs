use crate::protocol_serializer::ProtocolSerializer;
use crate::protocol_state::ProtocolState::*;
use crab_macros::{Packet, ProtocolSerializer};
use crab_protocol::buf::{ReadByteBuf, WriteByteBuf};
use crab_protocol::error::ProtocolError;

#[derive(Packet, Debug, Clone)]
#[packet(name = "remove_entities", id = 0x47, state = Play)]
pub struct RemoveEntitiesS2CPacket {
    pub entities_ids: Vec<i32>,
}
impl ProtocolSerializer for RemoveEntitiesS2CPacket {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        buf.put_var_int(self.entities_ids.len() as i32);
        for id in self.entities_ids.iter() {
            buf.put_var_int(*id);
        }
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        todo!()
    }
}
