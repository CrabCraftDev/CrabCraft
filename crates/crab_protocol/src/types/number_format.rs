use crab_nbt::NbtTag;
use crab_protocol::buf::{ReadByteBuf, WriteByteBuf};
use crab_protocol::error::ProtocolError;
use crab_protocol::protocol_serializer::ProtocolSerializer;
use crab_text::text::Text;

#[derive(Debug, PartialEq, Clone)]
pub enum NumberFormat {
    Blank,
    Styled(NbtTag),
    Fixed(Box<Text>),
}

impl ProtocolSerializer for NumberFormat {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        match self {
            NumberFormat::Blank => {
                buf.put_var_int(0);
            }
            NumberFormat::Styled(styling) => {
                buf.put_var_int(1);
                ProtocolSerializer::serialize(styling, buf)?;
            }
            NumberFormat::Fixed(text_component) => {
                buf.put_var_int(2);
                text_component.serialize(buf)?;
            }
        }
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        let number_format_type = buf.get_var_int()?;
        match number_format_type {
            0 => Ok(NumberFormat::Blank),
            1 => {
                let styling = ProtocolSerializer::deserialize(buf)?;
                Ok(NumberFormat::Styled(styling))
            }
            2 => {
                let text_component = Box::new(Text::deserialize(buf)?);
                Ok(NumberFormat::Fixed(text_component))
            }
            _ => Err(ProtocolError::EnumOutOfBounds(
                "NumberFormat".to_string(),
                number_format_type as usize,
            )),
        }
    }
}
