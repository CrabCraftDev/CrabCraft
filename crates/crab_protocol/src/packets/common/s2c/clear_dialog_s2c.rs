use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "clear_dialog", ids(Configuration = 0x11, Play = 0x84))]
pub struct ClearDialogS2CPacket;
