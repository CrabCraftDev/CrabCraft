use crate::protocol::ProtocolState::Handshake;
use crate::types::varint::VarInt;
use strum::FromRepr;

use crab_protocol_macros::{Packet, ProtocolType};

#[derive(Packet, Debug, Clone)]
#[packet(id = 0, state = Handshake)]
pub struct HandshakeC2SPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub protocol_version: i32,
    #[protocol_attribute(bounded = 255)]
    pub server_address: String,
    pub server_port: u16,
    pub next_state: NextState,
}

#[derive(ProtocolType, FromRepr, Copy, Clone, Debug)]
pub enum NextState {
    Status = 1,
    Login = 2,
}
