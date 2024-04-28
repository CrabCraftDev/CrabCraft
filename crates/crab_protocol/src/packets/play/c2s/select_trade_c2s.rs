use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x2A, state = Play)]
pub struct SelectTradeC2SPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub selected_slot: i32,
}
