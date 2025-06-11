use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "container_slot_state_changed", id = 0x13, state = Play)]
/// Used only for [Crafter block](https://minecraft.wiki/w/Crafter)
pub struct ChangeContainerSlotStateC2SPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub slot_id: i32,
    #[protocol_attribute(protocol_type = VarInt)]
    pub window_id: i32,
    pub is_enabled: bool,
}
