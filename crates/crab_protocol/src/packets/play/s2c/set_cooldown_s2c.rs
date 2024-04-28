use crab_protocol::protocol::ProtocolState::*;
use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::Packet;

#[derive(Debug, Packet, Clone)]
#[packet(id = 0x16, state = Play)]
pub struct SetCooldownS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub item_id: i32,
    #[protocol_attribute(protocol_type = VarInt)]
    pub cooldown_ticks: i32,
}
