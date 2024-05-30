use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::known_pack::KnownPack;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x07, state = Configuration)]
pub struct KnownPacksC2SPacket {
    pub known_packs: Vec<KnownPack>,
}
