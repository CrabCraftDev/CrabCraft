use uuid::Uuid;

use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::resource_pack_status::ResourcePackStatus;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x2B, state = Play)]
pub struct ResourcePackResponseC2SPacket {
    pub resource_pack_uuid: Uuid,
    pub result: ResourcePackStatus,
}
