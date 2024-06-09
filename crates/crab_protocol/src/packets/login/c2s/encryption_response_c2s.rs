use bytes::Bytes;

use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x01, state = Login)]
pub struct EncryptionResponseC2SPacket {
    pub shared_secret: Bytes,
    pub verify_token: Bytes,
}
