use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x2E, state = Play)]
pub struct ProgramCommandBlockMinecartC2SPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub entity_id: i32,
    pub command: String,
    pub track_output: bool,
}
