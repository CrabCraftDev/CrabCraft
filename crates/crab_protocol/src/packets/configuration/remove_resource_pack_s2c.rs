use uuid::Uuid;

use crate::protocol::ProtocolState::*;
use crab_protocol_macros::Packet;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x06, state = Configuration)]
pub struct RemoveResourcePackS2CPacket {
    pub resource_pack_uuid: Option<Uuid>,
}
