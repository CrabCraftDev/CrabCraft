use uuid::Uuid;

use crab_protocol::protocol::ProtocolState::*;
use crab_protocol::types::property::Property;
use crab_protocol_macros::Packet;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x02, state = Login)]
pub struct LoginSuccessS2CPacket {
    pub uuid: Uuid,
    #[protocol_attribute(bounded = 16)]
    pub username: String,
    pub properties: Vec<Property>,
}
