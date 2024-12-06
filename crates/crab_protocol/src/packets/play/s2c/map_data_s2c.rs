use bytes::Bytes;
use derive_more::{From, Into};
use strum::FromRepr;

use crab_macros::{Packet, ProtocolSerializer};

use crate::buf::{ReadByteBuf, WriteByteBuf};
use crate::error::ProtocolError;
use crate::protocol_serializer::ProtocolSerializer;
use crate::protocol_state::ProtocolState::*;
use crate::types::text::TextComponent;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "map_item_data", id = 0x2D, state = Play)]
pub struct MapDataS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub map_id: i32,
    pub scale: i8,
    pub is_locked: bool,
    pub command_suggestions: Option<Vec<MapDecoration>>,
    pub map_patches: OptionalMapPatch,
}

#[derive(Debug, Clone, PartialEq, ProtocolSerializer)]
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

impl ProtocolSerializer for OptionalMapPatch {
    fn serialize(&self, mut buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        match &self.0 {
            None => buf.put_u8(0),
            Some(map_patch) => ProtocolSerializer::serialize(map_patch, &mut buf)?,
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

#[derive(Debug, Clone, Hash, ProtocolSerializer)]
pub struct MapPatch {
    pub columns: u8,
    pub rows: u8,
    pub x: u8,
    pub z: u8,
    pub rotation: i8,
    pub data: Bytes,
}

#[derive(Debug, Copy, Clone, PartialEq, ProtocolSerializer, FromRepr)]
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
