use crate::protocol_state::ProtocolState::*;
use crab_macros::{Packet, ProtocolSerializer};
use crab_protocol::buf::{ReadByteBuf, WriteByteBuf};
use crab_protocol::error::ProtocolError;
use crab_protocol::protocol_serializer::ProtocolSerializer;
use crab_text::utils::namespaced_key::NamespacedKey;
use strum::FromRepr;
use uuid::Uuid;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "waypoint", id = 0x83, state = Play)]
pub struct WaypointS2CPacket {
    pub operation: WaypointOperation,
    pub waypoint: Waypoint,
}

#[derive(ProtocolSerializer, Debug, Clone)]
pub struct Waypoint {
    pub uuid: Uuid,
    pub icon: WaypointIcon,
    pub waypoint_type: WaypointType,
}

#[derive(ProtocolSerializer, Debug, Clone)]
pub struct WaypointIcon {
    pub style: NamespacedKey,
    pub color: RgbColor,
}
#[derive(ProtocolSerializer, Debug, Clone)]
pub struct RgbColor {
    pub r: i8,
    pub g: i8,
    pub b: i8,
}

#[derive(Debug, Copy, Clone, PartialEq, ProtocolSerializer, FromRepr)]
pub enum WaypointOperation {
    Track,
    Untrack,
    Update,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum WaypointType {
    Empty,
    Position { x: i32, y: i32, z: i32 },
    Chunk { chunk_x: i32, chunk_z: i32 },
    Azimuth { angle: f32 },
}

impl ProtocolSerializer for WaypointType {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        match self {
            WaypointType::Empty => {
                buf.put_u8(0);
            }
            WaypointType::Position { x, y, z } => {
                buf.put_u8(1);
                buf.put_var_int(*x);
                buf.put_var_int(*y);
                buf.put_var_int(*z);
            }
            WaypointType::Chunk { chunk_x, chunk_z } => {
                buf.put_u8(2);
                buf.put_i32(*chunk_x);
                buf.put_i32(*chunk_z);
            }
            WaypointType::Azimuth { angle } => {
                buf.put_u8(3);
                buf.put_f32(*angle);
            }
        }
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        let mode = buf.get_i8();
        match mode {
            0 => Ok(WaypointType::Empty),
            1 => {
                let x = buf.get_var_int()?;
                let y = buf.get_var_int()?;
                let z = buf.get_var_int()?;
                Ok(WaypointType::Position { x, y, z })
            }
            2 => {
                let chunk_x = buf.get_i32();
                let chunk_z = buf.get_i32();
                Ok(WaypointType::Chunk { chunk_x, chunk_z })
            }
            3 => {
                let angle = buf.get_f32();
                Ok(WaypointType::Azimuth { angle })
            }
            _ => Err(ProtocolError::EnumOutOfBounds(
                "WaypointType".to_string(),
                mode as usize,
            )),
        }
    }
}
