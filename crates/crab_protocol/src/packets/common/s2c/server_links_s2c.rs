use crate::buf::{ReadByteBuf, WriteByteBuf};
use crate::error::ProtocolError;
use crate::packet::Packet;
use crate::protocol_serializer::ProtocolSerializer;
use crate::protocol_state::ProtocolState;
use crab_macros::ProtocolSerializer;
use strum::FromRepr;

use crate::protocol_state::ProtocolState::*;
use crate::types::text::TextComponent;

#[derive(ProtocolSerializer, Debug, Clone)]
pub struct ServerLinksS2CPacket {
    pub links: Vec<ServerLink>,
}

impl Packet for ServerLinksS2CPacket {
    const NAME: &'static str = "server_links";
    const STATE_IDS: &'static [(ProtocolState, i32)] = &[(Play, 0x82), (Configuration, 0x10)];
}

#[derive(Debug, Copy, Clone, PartialEq, ProtocolSerializer, FromRepr)]
pub enum KnownServerLinkType {
    BugReport,
    CommunityGuidelines,
    Support,
    Status,
    Feedback,
    Community,
    Website,
    Forums,
    News,
    Announcements,
}

#[derive(Debug, Clone)]
pub enum ServerLink {
    Known { link: KnownServerLinkType, url: String },
    Custom { component: TextComponent, url: String },
}

impl ProtocolSerializer for ServerLink {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        match self {
            ServerLink::Known { link, url } => {
                buf.put_bool(true);
                link.serialize(buf)?;
                url.serialize(buf)?;
            }
            ServerLink::Custom { component, url } => {
                buf.put_bool(false);
                component.serialize(buf)?;
                url.serialize(buf)?;
            }
        }
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        let known = buf.get_bool();
        if known {
            let link = KnownServerLinkType::deserialize(buf)?;
            let url = String::deserialize(buf)?;
            Ok(ServerLink::Known { link, url })
        } else {
            let component = TextComponent::deserialize(buf)?;
            let url = String::deserialize(buf)?;
            Ok(ServerLink::Custom { component, url })
        }
    }
}
