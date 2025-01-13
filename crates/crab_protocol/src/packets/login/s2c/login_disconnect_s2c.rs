use crate::protocol_state::ProtocolState::*;
use crab_macros::Packet;
use crab_protocol::buf::{ReadByteBuf, WriteByteBuf};
use crab_protocol::error::ProtocolError;
use crab_protocol::protocol_serializer::ProtocolSerializer;
use crab_text::formats::json::JsonTextFormat;
use crab_text::formats::TextFormat;
use crab_text::text::Text;

#[derive(Packet, Debug, Clone)]
#[packet(name = "login_disconnect", id = 0x00, state = Login)]
pub struct LoginDisconnectS2CPacket {
    pub reason: Text,
}

impl ProtocolSerializer for LoginDisconnectS2CPacket {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        let json = JsonTextFormat.write(&self.reason)?;
        buf.put_string(&json);
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        let value = buf.get_string()?;
        Ok(LoginDisconnectS2CPacket {
            reason: JsonTextFormat.parse(&value)?,
        })
    }
}
