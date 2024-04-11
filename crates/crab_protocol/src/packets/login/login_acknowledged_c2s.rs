use crate::protocol::ProtocolState::*;

use crab_protocol_macros::Packet;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x03, state = Login)]
pub struct LoginAcknowledgedC2SPacket;
