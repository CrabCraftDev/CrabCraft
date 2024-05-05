use strum::FromRepr;

use crab_protocol::buf::{ReadByteBuf, WriteByteBuf};
use crab_protocol::error::ProtocolError;
use crab_protocol::protocol_serializer::ProtocolSerializer;
use crab_protocol::types::identifier::Identifier;
use crab_protocol_macros::ProtocolSerializer;

#[derive(Debug, Copy, Clone, PartialEq, ProtocolSerializer, FromRepr)]
pub enum SoundCategory {
    Master,
    Music,
    Records,
    Weather,
    Blocks,
    Hostile,
    Neutral,
    Players,
    Ambient,
    Voice,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SoundEffect {
    Direct {
        id: i32,
    },
    Identifier {
        sound_name: Identifier,
        fixed_range: Option<f32>,
    },
}

impl ProtocolSerializer for SoundEffect {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        match self {
            SoundEffect::Direct { id } => {
                buf.put_var_int(id + 1);
            }
            SoundEffect::Identifier {
                sound_name,
                fixed_range,
            } => {
                sound_name.serialize(buf)?;
                fixed_range.serialize(buf)?;
            }
        }
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        let id = buf.get_i32();
        if id == 0 {
            let sound_name = Identifier::deserialize(buf)?;
            let fixed_range = Option::<f32>::deserialize(buf)?;
            Ok(SoundEffect::Identifier {
                sound_name,
                fixed_range,
            })
        } else {
            Ok(SoundEffect::Direct { id: id - 1 })
        }
    }
}
