use bytes::Bytes;
use uuid::Uuid;

use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "chat_session_update", id = 0x09, state = Play)]
pub struct PlayerSessionC2SPacket {
    pub session_id: Uuid,
    pub expires_at: i64,
    #[protocol_attribute(bounded = 512)]
    pub public_key: Bytes,
    #[protocol_attribute(bounded = 4096)]
    pub key_signature: Bytes,
}
