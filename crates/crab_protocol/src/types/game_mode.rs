use derive_more::{From, Into};
use strum::FromRepr;

use crab_macros::ProtocolSerializer;
use crab_protocol::buf::{ReadByteBuf, WriteByteBuf};
use crab_protocol::error::ProtocolError;
use crab_protocol::protocol_serializer::ProtocolSerializer;

#[derive(Debug, Copy, Clone, PartialEq, FromRepr, ProtocolSerializer)]
#[repr(u8)]
pub enum GameMode {
    Survival = 0,
    Creative = 1,
    Adventure = 2,
    Spectator = 3,
}

#[derive(Copy, Clone, PartialEq, Default, Debug, From, Into)]
pub struct OptionalGameMode(pub Option<GameMode>);

impl ProtocolSerializer for OptionalGameMode {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        match self.0 {
            Some(game_mode) => buf.put_i8(game_mode as i8),
            None => buf.put_i8(-1),
        }
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        let value = buf.get_i8();
        match value {
            -1 => Ok(None.into()),
            _ => {
                let game_mode = GameMode::from_repr(value as u8)
                    .ok_or_else(|| ProtocolError::EnumOutOfBounds("OptionalGameMode".to_string(), value as usize))?;
                Ok(Some(game_mode).into())
            }
        }
    }
}
