use crab_macros::ProtocolSerializer;

use crate::packet::Packet;
use crate::protocol_state::ProtocolState;
use crate::protocol_state::ProtocolState::*;
use crab_text::utils::namespaced_key::NamespacedKey;

#[derive(ProtocolSerializer, Debug, Clone)]
pub struct CookieRequestS2CPacket {
    pub key: NamespacedKey,
}

impl Packet for CookieRequestS2CPacket {
    const NAME: &'static str = "cookie_request";
    const STATE_IDS: &'static [(ProtocolState, i32)] = &[(Configuration, 0x00), (Login, 0x05), (Play, 0x16)];
}
