use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "rename_item", id = 0x2F, state = Play)]
pub struct RenameItemC2SPacket {
    pub item_name: String,
}
