use uuid::Uuid;

use crab_protocol_macros::{Packet, ProtocolType};

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x02, state = Login)]
pub struct LoginSuccessS2CPacket {
    pub uuid: Uuid,
    #[protocol_attribute(bounded = 16)]
    pub username: String,
    pub properties: Vec<Property>,
}

#[derive(Debug, Clone, PartialEq, ProtocolType)]
pub struct Property {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}
