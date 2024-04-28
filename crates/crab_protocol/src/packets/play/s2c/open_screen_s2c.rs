use crab_protocol::protocol::ProtocolState::*;
use crab_protocol::types::text::TextComponent;
use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::Packet;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x31, state = Play)]
pub struct OpenScreenS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub window_id: i32,
    #[protocol_attribute(protocol_type = VarInt)]
    pub window_type: i32,
    pub title: TextComponent,
}
