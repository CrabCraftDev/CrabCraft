use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x2A, state = Play)]
pub struct RenameItemC2SPacket {
    pub item_name: String,
}
