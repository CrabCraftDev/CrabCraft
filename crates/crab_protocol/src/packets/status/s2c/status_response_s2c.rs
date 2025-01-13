use crab_macros::{Packet, ProtocolSerializer};
use serde::{Deserialize, Serialize};

use crate::buf::{ReadByteBuf, WriteByteBuf};
use crate::error::ProtocolError;
use crate::protocol_serializer::ProtocolSerializer;
use crate::protocol_state::ProtocolState::*;
use crab_text::text::Text;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "status_response", id = 0x00, state = Status)]
pub struct StatusResponseS2CPacket {
    pub server_info: ServerInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerInfo {
    pub version: ServerVersion,
    pub players: PlayersInfo,
    pub description: Text,
    pub favicon: String,
    pub enforces_secure_chat: bool,
    pub previews_chat: bool,
}

impl ProtocolSerializer for ServerInfo {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        buf.put_string(&serde_json::to_string(&self)?);
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        let content = buf.get_string()?;
        Ok(serde_json::from_str(&content)?)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerVersion {
    pub name: String,
    #[serde(rename = "protocol")]
    pub protocol_version: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayersInfo {
    pub max: i32,
    pub online: i32,
    pub sample: Vec<PlayerSample>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerSample {
    pub name: String,
    pub uuid: String,
}
