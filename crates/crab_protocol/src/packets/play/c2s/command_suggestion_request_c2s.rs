use crab_protocol::protocol::ProtocolState::*;
use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::Packet;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x0A, state = Play)]
pub struct CommandSuggestionRequestC2SPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub transaction_id: i32,
    #[protocol_attribute(bounded = 32500)]
    pub text: String,
}
