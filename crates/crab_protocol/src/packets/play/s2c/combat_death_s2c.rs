use crab_protocol::types::text::TextComponent;
use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x3A, state = Play)]
pub struct CombatDeathS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub player_id: i32,
    pub message: TextComponent,
}
