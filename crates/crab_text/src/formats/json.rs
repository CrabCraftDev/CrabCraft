use crate::error::TextError;
use crate::formats::TextFormat;
use crate::text::Text;
use serde_json::{from_str, to_string};

pub const MAX_JSON_TEXT_COMPONENT_LENGTH: usize = i16::MAX as usize;

pub struct JsonTextFormat;

impl TextFormat for JsonTextFormat {
    type TargetFormatConversion = String;

    fn parse(&self, input: &String) -> Result<Text, TextError> {
        if input.len() > MAX_JSON_TEXT_COMPONENT_LENGTH {
            return Err(TextError::JsonTextComponentTooLong);
        }

        Ok(from_str(input)?)
    }

    fn write(&self, text: &Text) -> Result<String, TextError> {
        let serialized_text = to_string(text)?;
        if serialized_text.len() > MAX_JSON_TEXT_COMPONENT_LENGTH {
            return Err(TextError::JsonTextComponentTooLong);
        }

        Ok(serialized_text)
    }
}
