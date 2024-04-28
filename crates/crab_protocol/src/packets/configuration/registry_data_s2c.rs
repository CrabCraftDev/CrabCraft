use crab_nbt::Nbt;

use crate::protocol::ProtocolState::*;
use crab_protocol_macros::Packet;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x05, state = Configuration)]
pub struct RegistryDataS2CPacket {
    pub registry_data: Nbt,
}
