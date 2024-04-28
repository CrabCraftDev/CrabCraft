use bitfield_struct::bitfield;

use crab_protocol::buf::{ReadByteBuf, WriteByteBuf};
use crab_protocol::error::ProtocolError;
use crab_protocol::packets::play::LastDeathLocation;
use crab_protocol::protocol_type::ProtocolType;
use crab_protocol::types::game_mode::GameMode;
use crab_protocol::types::game_mode::OptionalGameMode;
use crab_protocol::types::identifier::Identifier;
use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x45, state = Play)]
pub struct RespawnS2CPacket {
    pub dimension_type: Identifier,
    pub dimension_name: Identifier,
    pub hashed_seed: i64,
    pub game_mode: GameMode,
    #[protocol_attribute(protocol_type = OptionalGameMode)]
    pub previous_game_mode: Option<GameMode>,
    pub is_debug: bool,
    pub is_flat: bool,
    pub last_death_location: Option<LastDeathLocation>,
    #[protocol_attribute(protocol_type = VarInt)]
    pub portal_cooldown: i32,
    pub data_kept: DataKeptAfterRespawn,
}

#[bitfield(u8)]
pub struct DataKeptAfterRespawn {
    pub keep_attributes: bool,
    pub keep_meta: bool,
    #[bits(6)]
    __: u8,
}

impl ProtocolType for DataKeptAfterRespawn {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        buf.put_i8(self.0 as i8);
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        Ok(DataKeptAfterRespawn(buf.get_i8() as u8))
    }
}
