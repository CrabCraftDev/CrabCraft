use bytes::Bytes;

use crab_macros::ProtocolSerializer;

use crate::packet::Packet;
use crate::protocol_state::ProtocolState;
use crate::protocol_state::ProtocolState::*;
use crate::types::namespaced_key::NamespacedKey;

#[derive(ProtocolSerializer, Debug, Clone)]
pub struct CookieResponseC2SPacket {
    pub key: NamespacedKey,
    #[protocol_attribute(bounded = 5120)]
    pub payload: Option<Bytes>,
}

impl Packet for CookieResponseC2SPacket {
    const NAME: &'static str = "cookie_response";
    const STATE_IDS: &'static [(ProtocolState, i32)] = &[(Configuration, 0x01), (Login, 0x04), (Play, 0x13)];
}
