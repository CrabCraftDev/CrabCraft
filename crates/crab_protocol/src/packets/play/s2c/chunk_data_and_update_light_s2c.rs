use bytes::Bytes;
use crab_nbt::Nbt;

use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::buf::{ReadByteBuf, WriteByteBuf};
use crate::error::ProtocolError;
use crate::packets::play::s2c::update_light_s2c::LightData;
use crate::protocol::ProtocolState::*;
use crate::protocol_serializer::ProtocolSerializer;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x25, state = Play)]
pub struct ChunkDataAndUpdateLightS2CPacket {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub heightmaps: Nbt,
    pub data: Bytes,
    pub block_entities: Vec<BlockEntity>,
    pub light_data: LightData,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockEntity {
    pub relative_x: i8,
    pub relative_z: i8,
    pub y: i16,
    pub block_entity_type: i32,
    pub data: Nbt,
}

impl ProtocolSerializer for BlockEntity {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        buf.put_i8(((self.relative_x & 15) << 4) | (self.relative_z & 0x0F));
        buf.put_i16(self.y);
        buf.put_i32(self.block_entity_type);
        self.data.serialize(buf)?;
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        let packed_xz = buf.get_i8();
        let x = packed_xz >> 4;
        let z = packed_xz & 0x0F;
        Ok(BlockEntity {
            relative_x: x,
            relative_z: z,
            y: buf.get_i16(),
            block_entity_type: buf.get_i32(),
            data: Nbt::deserialize(buf)?,
        })
    }
}
