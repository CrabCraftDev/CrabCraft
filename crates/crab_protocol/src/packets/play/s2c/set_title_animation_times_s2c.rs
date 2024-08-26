use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x66, state = Play)]
pub struct SetTitleAnimationTimesS2CPacket {
    pub fade_in: i32,
    pub stay: i32,
    pub fade_out: i32,
}
