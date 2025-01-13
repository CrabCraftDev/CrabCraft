use crate::error::TextError;
use crate::formats::TextFormat;
use crate::text::Text;
use bytes::{Bytes, BytesMut};
use crab_nbt::serde::de::from_bytes_unnamed;
use crab_nbt::serde::ser::to_bytes_unnamed;

pub struct NbtTextFormat;

impl TextFormat for NbtTextFormat {
    type TargetFormatConversion = Bytes;

    fn parse(&self, input: &Bytes) -> Result<Text, TextError> {
        Ok(from_bytes_unnamed(&mut BytesMut::from(input.clone()))?)
    }

    fn write(&self, text: &Text) -> Result<Bytes, TextError> {
        Ok(to_bytes_unnamed(text)?.freeze())
    }
}
