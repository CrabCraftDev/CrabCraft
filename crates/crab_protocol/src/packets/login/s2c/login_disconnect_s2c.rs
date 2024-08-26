use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::text::JsonTextComponent;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x00, state = Login)]
pub struct LoginDisconnectS2CPacket {
    pub reason: JsonTextComponent,
}
