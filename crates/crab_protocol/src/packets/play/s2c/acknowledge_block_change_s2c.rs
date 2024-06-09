use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x05, state = Play)]
pub struct AcknowledgeBlockChangeS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub sequence_id: i32,
}
