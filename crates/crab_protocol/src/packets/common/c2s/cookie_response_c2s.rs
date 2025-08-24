use bytes::Bytes;

use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crab_text::utils::namespaced_key::NamespacedKey;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "cookie_response", ids(Configuration = 0x01, Login = 0x04, Play = 0x14))]
pub struct CookieResponseC2SPacket {
    pub key: NamespacedKey,
    #[protocol_attribute(bounded = 5120)]
    pub payload: Option<Bytes>,
}
