use crate::error::TextError;
use crate::formats::TextFormat;
use crate::text::Text;
use crab_nbt::serde::de::from_bytes_unnamed;
use crab_nbt::serde::ser::to_bytes_unnamed;

pub struct NbtTextFormat;

impl TextFormat for NbtTextFormat {
    type TargetFormatConversion = Vec<u8>;

    fn parse(&self, input: &Vec<u8>) -> Result<Text, TextError> {
        Ok(from_bytes_unnamed(input)?)
    }

    fn write(&self, text: &Text) -> Result<Vec<u8>, TextError> {
        Ok(to_bytes_unnamed(text)?)
    }
}
