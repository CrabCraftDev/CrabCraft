use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::known_pack::KnownPack;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "select_known_packs", id = 0x07, state = Configuration)]
pub struct KnownPacksC2SPacket {
    pub known_packs: Vec<KnownPack>,
}
