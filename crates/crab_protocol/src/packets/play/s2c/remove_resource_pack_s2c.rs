use uuid::Uuid;

use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x43, state = Play)]
pub struct RemoveResourcePackS2CPacket {
    pub resource_pack_uuid: Option<Uuid>,
}
