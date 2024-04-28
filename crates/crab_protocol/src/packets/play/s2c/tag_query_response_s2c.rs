use crab_nbt::NbtTag;

use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x6B, state = Play)]
pub struct TagQueryResponseS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub transaction_id: i32,
    pub nbt: NbtTag,
}
