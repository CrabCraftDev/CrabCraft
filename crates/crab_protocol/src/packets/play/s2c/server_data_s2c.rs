use bytes::Bytes;

use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::text::TextComponent;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x49, state = Play)]
pub struct ServerDataS2CPacket {
    pub motd: TextComponent,
    pub icon: Option<Bytes>,
    pub enforces_secure_chat: bool,
}
