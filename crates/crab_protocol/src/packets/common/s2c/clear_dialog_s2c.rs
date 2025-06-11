use crab_macros::ProtocolSerializer;

use crate::packet::Packet;
use crate::protocol_state::ProtocolState;
use crate::protocol_state::ProtocolState::*;

#[derive(ProtocolSerializer, Debug, Clone)]
pub struct ClearDialogS2CPacket;

impl Packet for ClearDialogS2CPacket {
    const NAME: &'static str = "clear_dialog";
    const STATE_IDS: &'static [(ProtocolState, i32)] = &[(Configuration, 0x11), (Play, 0x84)];
}
