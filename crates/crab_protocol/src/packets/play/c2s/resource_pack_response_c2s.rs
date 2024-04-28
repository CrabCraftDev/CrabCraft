use uuid::Uuid;

use crab_protocol::types::resource_pack_status::ResourcePackStatus;
use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x28, state = Play)]
pub struct ResourcePackResponseC2SPacket {
    pub resource_pack_uuid: Uuid,
    pub result: ResourcePackStatus,
}
