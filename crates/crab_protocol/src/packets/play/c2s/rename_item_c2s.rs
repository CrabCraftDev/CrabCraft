use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x27, state = Play)]
pub struct RenameItemC2SPacket {
    pub item_name: String,
}
