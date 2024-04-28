use bytes::Bytes;

use crab_protocol::types::text::TextComponent;
use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x49, state = Play)]
pub struct ServerDataS2CPacket {
    pub motd: TextComponent,
    pub icon: Option<Bytes>,
    pub enforces_secure_chat: bool,
}
