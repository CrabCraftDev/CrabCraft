use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x1D, state = Play)]
pub struct PickItemC2SPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub slot_to_use: i32,
}
