use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "command_suggestion", id = 0x0E, state = Play)]
pub struct CommandSuggestionRequestC2SPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub transaction_id: i32,
    #[protocol_attribute(bounded = 32500)]
    pub text: String,
}
