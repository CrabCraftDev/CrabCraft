use crab_protocol::types::hand::Hand;
use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x36, state = Play)]
pub struct UseItemC2SPacket {
    pub hand: Hand,
    #[protocol_attribute(protocol_type = VarInt)]
    pub sequence: i32,
}
