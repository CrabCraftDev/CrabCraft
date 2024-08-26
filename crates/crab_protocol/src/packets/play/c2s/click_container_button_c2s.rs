use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x0D, state = Play)]
pub struct ClickContainerButtonC2SPacket {
    pub window_id: i8,
    pub button_id: i8,
}
