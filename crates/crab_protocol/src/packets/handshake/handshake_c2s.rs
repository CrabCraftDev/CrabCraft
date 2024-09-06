use strum::FromRepr;

use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x00, state = Handshake)]
pub struct HandshakeC2SPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub protocol_version: i32,
    #[protocol_attribute(bounded = 255)]
    pub server_address: String,
    pub server_port: u16,
    pub next_state: NextState,
}

#[derive(ProtocolSerializer, FromRepr, Copy, Clone, Debug, Eq, PartialEq)]
pub enum NextState {
    Status = 1,
    Login = 2,
    Transfer = 3,
}
