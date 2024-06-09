use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x10, state = Play)]
/// Used only for [Crafter block](https://minecraft.wiki/w/Crafter)
pub struct ChangeContainerSlotStateC2SPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub slot_id: i32,
    #[protocol_attribute(protocol_type = VarInt)]
    pub window_id: i32,
    pub is_enabled: bool,
}
