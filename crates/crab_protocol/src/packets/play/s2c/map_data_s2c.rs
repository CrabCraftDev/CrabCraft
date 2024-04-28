use bytes::Bytes;
use derive_more::{From, Into};
use strum::FromRepr;

use crab_protocol::buf::{ReadByteBuf, WriteByteBuf};
use crab_protocol::error::ProtocolError;
use crab_protocol::protocol::ProtocolState::*;
use crab_protocol::protocol_type::ProtocolType;
use crab_protocol::types::text::TextComponent;
use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::{Packet, ProtocolType};

#[derive(Debug, Packet, Clone)]
#[packet(id = 0x2A, state = Play)]
pub struct MapDataS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub map_id: i32,
    pub scale: i8,
    pub is_locked: bool,
    pub command_suggestions: Option<Vec<MapDecoration>>,
    pub map_patches: OptionalMapPatch,
}

#[derive(Debug, Clone, PartialEq, ProtocolType)]
pub struct MapDecoration {
    pub decoration_type: MapDecorationType,
    pub x: i8,
    pub y: i8,
    pub rotation: i8,
    pub display_name: Option<TextComponent>,
}

#[derive(Clone, Default, Debug, From, Into)]
#[repr(transparent)]
pub struct OptionalMapPatch(pub Option<MapPatch>);

impl ProtocolType for OptionalMapPatch {
    fn serialize(&self, mut buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        match &self.0 {
            None => buf.put_u8(0),
            Some(map_patch) => ProtocolType::serialize(map_patch, &mut buf)?,
        }
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        let columns = buf.chunk()[0];
        Ok(Self(if columns == 0 {
            None
        } else {
            Some(MapPatch::deserialize(buf)?)
        }))
    }
}

#[derive(Debug, Clone, Hash, ProtocolType)]
pub struct MapPatch {
    pub columns: u8,
    pub rows: u8,
    pub x: u8,
    pub z: u8,
    pub rotation: i8,
    pub data: Bytes,
}

#[derive(Debug, Copy, Clone, PartialEq, ProtocolType, FromRepr)]
pub enum MapDecorationType {
    /// Player
    WhiteArrow,
    /// Green arrow
    GreenArrow,
    RedArrow,
    BlueArrow,
    WhiteCross,
    RedPointer,
    PlayerOffMap,
    PlayerOffLimit,
    Mansion,
    Monument,
    WhiteBanner,
    OrangeBanner,
    MagentaBanner,
    LightBlueBanner,
    YellowBanner,
    LimeBanner,
    PinkBanner,
    GrayBanner,
    LightGrayBanner,
    CyanBanner,
    PurpleBanner,
    BlueBanner,
    BrownBanner,
    GreenBanner,
    RedBanner,
    BlackBanner,
    TreasureMarker,
}
