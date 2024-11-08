use crate::buf::{ReadByteBuf, WriteByteBuf};
use crate::error::ProtocolError;
use crate::protocol_serializer::ProtocolSerializer;
use crab_macros::ProtocolSerializer;
use derive_more::{From, Into};
use evenio::component::Component;

#[derive(Clone, Copy, PartialEq, Debug, ProtocolSerializer, Component, Default)]
pub struct Rotation {
    pub yaw: f32,
    pub pitch: f32,
}

impl Rotation {
    pub fn new(yaw: f32, pitch: f32) -> Self {
        Self { yaw, pitch }
    }
}

#[derive(Clone, Copy, PartialEq, Debug, ProtocolSerializer, Default)]
pub struct Vec3Double {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3Double {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

#[repr(transparent)]
#[derive(ProtocolSerializer, Debug, Clone, Component, From, Into, Copy)]
pub struct Position(pub Vec3Double);

#[repr(transparent)]
#[derive(Debug, Clone, Component, From, Into, Copy)]
pub struct Velocity(pub Vec3Double);

const VELOCITY_PROTOCOL_MULTIPLIER: f64 = 8000.;

/// > Velocity is in units of 1/8000 of a block per server tick (50ms normal);
/// > for example, -1343 would move (-1343 / 8000) = −0.167875 blocks per tick (or −3.3575 blocks per second).
///
/// Description taken from https://wiki.vg/Protocol#Set_Entity_Velocity
impl ProtocolSerializer for Velocity {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        let vec = self.0;
        buf.put_i16((vec.x * VELOCITY_PROTOCOL_MULTIPLIER) as i16);
        buf.put_i16((vec.y * VELOCITY_PROTOCOL_MULTIPLIER) as i16);
        buf.put_i16((vec.z * VELOCITY_PROTOCOL_MULTIPLIER) as i16);
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        Ok(Vec3Double {
            x: buf.get_i16() as f64 / VELOCITY_PROTOCOL_MULTIPLIER,
            y: buf.get_i16() as f64 / VELOCITY_PROTOCOL_MULTIPLIER,
            z: buf.get_i16() as f64 / VELOCITY_PROTOCOL_MULTIPLIER,
        }
        .into())
    }
}

impl Default for Velocity {
    fn default() -> Self {
        Self(Vec3Double::new(0f64, 0f64, 0f64))
    }
}
