use crab_protocol::protocol::ProtocolState::*;
use crab_protocol::types::text::JsonTextComponent;
use crab_protocol_macros::Packet;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x00, state = Login)]
pub struct DisconnectS2CPacket {
    pub reason: JsonTextComponent,
}
