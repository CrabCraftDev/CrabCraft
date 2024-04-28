use uuid::Uuid;

use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x43, state = Play)]
pub struct RemoveResourcePackS2CPacket {
    pub resource_pack_uuid: Option<Uuid>,
}
