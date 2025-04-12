use crate::error::TextError;
use crate::text::Text;

pub mod json;
pub mod legacy;
pub mod nbt;
pub mod ratatui_text;
pub mod terminal;

/// A trait for converting text between different formats.
///
/// This trait defines methods for parsing text from a specific format
/// and writing text into that format. Formats could include JSON, NBT,
/// or other configuration formats such as Legacy, MiniMessage, or custom-defined ones.
pub trait TextFormat {
    /// Represents the specific format or type that the implementation supports for input and output operations.
    type TargetFormatConversion;

    fn parse(&self, input: &Self::TargetFormatConversion) -> Result<Text, TextError>;
    fn write(&self, text: &Text) -> Result<Self::TargetFormatConversion, TextError>;
}
