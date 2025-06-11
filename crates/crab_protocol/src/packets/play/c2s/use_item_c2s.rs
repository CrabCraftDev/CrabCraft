use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::hand::Hand;
use crate::types::position::Rotation;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "use_item", id = 0x40, state = Play)]
pub struct UseItemC2SPacket {
    pub hand: Hand,
    #[protocol_attribute(protocol_type = VarInt)]
    pub sequence: i32,
    pub rotation: Rotation,
}
