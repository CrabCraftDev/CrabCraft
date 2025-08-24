use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crab_text::text::Text;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "disconnect", ids(Configuration = 0x02, Play = 0x1C))]
pub struct DisconnectS2CPacket {
    pub reason: Text,
}
