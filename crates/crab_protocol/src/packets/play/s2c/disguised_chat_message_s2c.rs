use crab_protocol::protocol::ProtocolState::*;
use crab_protocol::types::text::TextComponent;
use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::Packet;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x1C, state = Play)]
pub struct DisguisedChatMessageS2CPacket {
    pub message: TextComponent,
    #[protocol_attribute(protocol_type = VarInt)]
    pub chat_type: i32,
    pub sender_name: TextComponent,
    pub target_name: Option<TextComponent>,
}
