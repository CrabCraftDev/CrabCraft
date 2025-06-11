use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "block_changed_ack", id = 0x04, state = Play)]
pub struct AcknowledgeBlockChangeS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub sequence_id: i32,
}
