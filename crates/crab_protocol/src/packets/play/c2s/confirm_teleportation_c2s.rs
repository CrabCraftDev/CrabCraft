use crab_protocol::protocol::ProtocolState::*;
use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::Packet;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x00, state = Play)]
pub struct ConfirmTeleportationC2SPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub teleport_id: i32,
}
