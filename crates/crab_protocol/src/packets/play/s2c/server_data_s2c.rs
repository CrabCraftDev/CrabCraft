use bytes::Bytes;

use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::text::TextComponent;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "server_data", id = 0x50, state = Play)]
pub struct ServerDataS2CPacket {
    pub motd: TextComponent,
    pub icon: Option<Bytes>,
    pub enforces_secure_chat: bool,
}
