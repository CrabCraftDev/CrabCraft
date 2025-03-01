use crab_macros::{Packet, ProtocolSerializer};

use crate::buf::{ReadByteBuf, WriteByteBuf};
use crate::error::ProtocolError;
use crate::protocol_serializer::ProtocolSerializer;
use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "section_blocks_update", id = 0x4E, state = Play)]
pub struct UpdateSectionBlocksS2CPacket {
    pub chunk_section_position: ChunkSectionPosition,
    pub blocks: Vec<BlockPositionAndState>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChunkSectionPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl ProtocolSerializer for ChunkSectionPosition {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        let x = self.x as i64;
        let y = self.y as i64;
        let z = self.z as i64;
        buf.put_i64(((x & 0x3FFFFF) << 42) | (y & 0xFFFFF) | ((z & 0x3FFFFF) << 20));
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        let val = buf.get_i64();
        let x = (val >> 42) as i32;
        let y = (val << 44 >> 44) as i32;
        let z = (val << 22 >> 42) as i32;

        Ok(Self { x, y, z })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockPositionAndState {
    pub state_id: i32,
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

impl ProtocolSerializer for BlockPositionAndState {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        let state_id = self.state_id as i64;
        let x = self.x as i64;
        let y = self.y as i64;
        let z = self.z as i64;
        buf.put_i64((state_id << 12) | ((x << 8) | (z << 4) | y));
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        let val = buf.get_i64();
        let state_id = (val >> 12) as i32;
        let x = ((val >> 8) & 0xF) as u8;
        let y = (val & 0xF) as u8;
        let z = ((val >> 4) & 0xF) as u8;

        Ok(Self { state_id, x, y, z })
    }
}
