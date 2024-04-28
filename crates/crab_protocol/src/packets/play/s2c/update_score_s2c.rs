use crab_protocol::types::number_format::NumberFormat;
use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;
use crate::types::text::TextComponent;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x5F, state = Play)]
pub struct UpdateScoreS2CPacket {
    /// For players, this is their username; for other entities, it is their UUID.
    pub entity_name: String,
    pub objective_name: String,
    #[protocol_attribute(protocol_type = VarInt)]
    pub value: i32,
    pub display_name: Option<TextComponent>,
    pub number_format: Option<NumberFormat>,
}
