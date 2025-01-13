use crab_macros::ProtocolSerializer;

use crate::packet::Packet;
use crate::protocol_state::ProtocolState;
use crate::protocol_state::ProtocolState::*;
use crab_text::text::Text;

#[derive(ProtocolSerializer, Debug, Clone)]
pub struct DisconnectS2CPacket {
    pub reason: Text,
}

impl Packet for DisconnectS2CPacket {
    const NAME: &'static str = "disconnect";
    const STATE_IDS: &'static [(ProtocolState, i32)] = &[(Configuration, 0x02), (Play, 0x1D)];
}
