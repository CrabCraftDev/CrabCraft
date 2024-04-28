use crab_nbt::NbtTag;
use crab_protocol::buf::{ReadByteBuf, WriteByteBuf};
use crab_protocol::error::ProtocolError;
use crab_protocol::protocol_type::ProtocolType;
use crab_protocol::types::text::TextComponent;

#[derive(Debug, PartialEq, Clone)]
pub enum NumberFormat {
    Blank,
    Styled(NbtTag),
    Fixed(TextComponent),
}

impl ProtocolType for NumberFormat {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        match self {
            NumberFormat::Blank => {
                buf.put_var_int(0);
            }
            NumberFormat::Styled(styling) => {
                buf.put_var_int(1);
                ProtocolType::serialize(styling, buf)?;
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
                let styling = ProtocolType::deserialize(buf)?;
                Ok(NumberFormat::Styled(styling))
            }
            2 => {
                let text_component = TextComponent::deserialize(buf)?;
                Ok(NumberFormat::Fixed(text_component))
            }
            _ => Err(ProtocolError::EnumOutOfBounds(
                "NumberFormat".to_string(),
                number_format_type as usize,
            )),
        }
    }
}
