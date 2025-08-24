use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crab_text::utils::namespaced_key::NamespacedKey;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "cookie_request", ids(Configuration = 0x00, Login = 0x05, Play = 0x15))]
pub struct CookieRequestS2CPacket {
    pub key: NamespacedKey,
}
