use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x00, state = Play)]
pub struct ConfirmTeleportationC2SPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub teleport_id: i32,
}
