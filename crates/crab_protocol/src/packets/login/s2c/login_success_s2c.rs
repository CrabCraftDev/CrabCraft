use uuid::Uuid;

use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::property::Property;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x02, state = Login)]
pub struct LoginSuccessS2CPacket {
    pub uuid: Uuid,
    #[protocol_attribute(bounded = 16)]
    pub username: String,
    pub properties: Vec<Property>,
    pub strict_protocol_error_handling: bool,
}
