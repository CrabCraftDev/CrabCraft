use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x2A, state = Play)]
pub struct RenameItemC2SPacket {
    pub item_name: String,
}
