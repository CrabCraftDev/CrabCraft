use crab_nbt::Nbt;

use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x05, state = Configuration)]
pub struct RegistryDataS2CPacket {
    pub registry_data: Nbt,
}
