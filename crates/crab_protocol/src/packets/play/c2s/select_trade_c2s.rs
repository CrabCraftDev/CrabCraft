use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x2D, state = Play)]
pub struct SelectTradeC2SPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub selected_slot: i32,
}
