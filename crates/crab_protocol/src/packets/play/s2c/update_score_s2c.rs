use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::number_format::NumberFormat;
use crate::types::text::TextComponent;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x61, state = Play)]
pub struct UpdateScoreS2CPacket {
    /// For players, this is their username; for other entities, it is their UUID.
    pub entity_name: String,
    pub objective_name: String,
    #[protocol_attribute(protocol_type = VarInt)]
    pub value: i32,
    pub display_name: Option<TextComponent>,
    pub number_format: Option<NumberFormat>,
}
