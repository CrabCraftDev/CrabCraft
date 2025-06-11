use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "set_command_minecart", id = 0x36, state = Play)]
pub struct ProgramCommandBlockMinecartC2SPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub entity_id: i32,
    pub command: String,
    pub track_output: bool,
}
