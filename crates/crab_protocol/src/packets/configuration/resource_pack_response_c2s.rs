use uuid::Uuid;

use crate::protocol::ProtocolState::*;
use crate::types::resource_pack_status::ResourcePackStatus;
use crab_protocol_macros::Packet;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x05, state = Configuration)]
pub struct ResourcePackResponseC2SPacket {
    pub resource_pack_uuid: Uuid,
    pub result: ResourcePackStatus,
}
