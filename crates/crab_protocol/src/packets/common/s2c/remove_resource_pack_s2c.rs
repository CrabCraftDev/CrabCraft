use crate::packet::Packet;
use crate::protocol::{PacketDirection, ProtocolState};
use uuid::Uuid;

use crab_protocol_macros::ProtocolSerializer;

use crate::protocol::ProtocolState::*;

#[derive(ProtocolSerializer, Debug, Clone)]
pub struct RemoveResourcePackS2CPacket {
    pub resource_pack_uuid: Option<Uuid>,
}

impl Packet for RemoveResourcePackS2CPacket {
    const SIDE: PacketDirection = PacketDirection::Clientbound;
    const STATE_IDS: &'static [(ProtocolState, i32)] = &[(Play, 0x45), (Status, 0x08)];
}
