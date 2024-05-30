use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x3A, state = Play)]
/// Unused by the Minecraft client.
pub struct EndCombatS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub duration: i32,
}
