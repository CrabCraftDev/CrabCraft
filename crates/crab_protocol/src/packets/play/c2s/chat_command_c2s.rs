use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x04, state = Play)]
pub struct ChatCommandC2SPacket {
    #[protocol_attribute(bounded = 256)]
    pub command: String,
}
