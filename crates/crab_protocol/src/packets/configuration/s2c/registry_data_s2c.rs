use crab_nbt::Nbt;

use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::identifier::Identifier;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x07, state = Configuration)]
pub struct RegistryDataS2CPacket {
    pub registry_id: Identifier,
    pub entries: Vec<RegistryDataEntry>,
}

#[derive(ProtocolSerializer, Debug, Clone)]
pub struct RegistryDataEntry {
    pub id: Identifier,
    pub data: Option<Nbt>,
}
