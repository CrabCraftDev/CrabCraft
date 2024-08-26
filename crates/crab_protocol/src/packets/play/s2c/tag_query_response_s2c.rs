use crab_nbt::NbtTag;

use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x6E, state = Play)]
pub struct TagQueryResponseS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub transaction_id: i32,
    pub nbt: NbtTag,
}
