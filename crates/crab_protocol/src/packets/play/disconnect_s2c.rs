use crab_nbt::NbtTag;
use crab_protocol::protocol::ProtocolState::*;
use crab_protocol_macros::Packet;

#[derive(Debug, Packet, Clone)]
#[packet(id = 0x1B, state = Configuration)]
pub struct DisconnectS2CPacket {
    pub reason: NbtTag,
}
