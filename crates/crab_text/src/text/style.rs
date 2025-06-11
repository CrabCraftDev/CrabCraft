use crate::text::color::TextColor;
use crate::text::events::{ClickEvent, HoverEvent};
use crate::utils::namespaced_key::NamespacedKey;
use crab_nbt::serde::bool::deserialize_option_bool;
use serde::{Deserialize, Serialize};

/// Describes text style properties
/// If a property is set to None, the parent's property is used instead.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct TextStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<TextColor>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_bool"
    )]
    pub bold: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_bool"
    )]
    pub italic: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_bool"
    )]
    pub underlined: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_bool"
    )]
    pub strikethrough: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_bool"
    )]
    pub obfuscated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<NamespacedKey>,
    /// Text to be inserted to chat when shift clicked on a component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insertion: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "click_event")]
    pub click_event: Option<ClickEvent>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hover_event")]
    pub hover_event: Option<HoverEvent>,
}

impl TextStyle {
    pub fn is_empty(&self) -> bool {
        self.color.is_none()
            && self.bold.is_none()
            && self.italic.is_none()
            && self.underlined.is_none()
            && self.strikethrough.is_none()
            && self.obfuscated.is_none()
            && self.font.is_none()
            && self.insertion.is_none()
            && self.click_event.is_none()
            && self.hover_event.is_none()
    }
}
