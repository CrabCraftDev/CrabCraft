use bytes::Bytes;

use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crab_text::text::Text;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "server_data", id = 0x4F, state = Play)]
pub struct ServerDataS2CPacket {
    pub motd: Text,
    pub icon: Option<Bytes>,
    pub enforces_secure_chat: bool,
}
