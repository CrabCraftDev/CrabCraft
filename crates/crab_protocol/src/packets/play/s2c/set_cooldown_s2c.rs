use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::namespaced_key::NamespacedKey;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "cooldown", id = 0x17, state = Play)]
pub struct SetCooldownS2CPacket {
    pub cooldown_group: NamespacedKey,
    #[protocol_attribute(protocol_type = VarInt)]
    pub cooldown_ticks: i32,
}
