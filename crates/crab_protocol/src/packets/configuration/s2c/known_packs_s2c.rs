use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::known_pack::KnownPack;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x0E, state = Configuration)]
pub struct KnownPacksS2CPacket {
    pub known_packs: Vec<KnownPack>,
}
