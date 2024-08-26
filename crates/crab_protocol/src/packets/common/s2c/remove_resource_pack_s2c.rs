use uuid::Uuid;

use crab_macros::ProtocolSerializer;

use crate::packet::Packet;
use crate::protocol_state::ProtocolState;
use crate::protocol_state::ProtocolState::*;

#[derive(ProtocolSerializer, Debug, Clone)]
pub struct RemoveResourcePackS2CPacket {
    pub resource_pack_uuid: Option<Uuid>,
}

impl Packet for RemoveResourcePackS2CPacket {
    const STATE_IDS: &'static [(ProtocolState, i32)] = &[(Play, 0x45), (Configuration, 0x08)];
}
