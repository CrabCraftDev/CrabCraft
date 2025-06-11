use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::number_format::NumberFormat;
use crate::types::varint::VarInt;
use crab_text::text::Text;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "set_score", id = 0x67, state = Play)]
pub struct UpdateScoreS2CPacket {
    /// For players, this is their username; for other entities, it is their UUID.
    pub entity_name: String,
    pub objective_name: String,
    #[protocol_attribute(protocol_type = VarInt)]
    pub value: i32,
    pub display_name: Option<Text>,
    pub number_format: Option<NumberFormat>,
}
