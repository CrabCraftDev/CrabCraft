use crate::error::TextError;
use crate::text::color::TextColor;
use crate::text::content::TextContent;
use crate::text::events::{ClickEvent, HoverEvent};
use crate::text::style::TextStyle;
use crate::utils::namespaced_key::NamespacedKey;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::str::FromStr;

pub mod color;
pub mod content;
pub mod events;
pub mod style;

/// The `Text` struct represents styled text with support for formatting and nested components.
///
/// For more details on text formatting, see [Minecraft Formatting Codes](https://minecraft.wiki/w/Formatting_codes).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct Text {
    #[serde(flatten)]
    pub content: TextContent,
    #[serde(flatten)]
    pub style: TextStyle,
    #[serde(default, skip_serializing_if = "Vec::is_empty", rename = "extra")]
    pub children: Vec<Text>,
}

impl Text {
    pub fn literal(text: impl Into<Cow<'static, str>>) -> Self {
        Text::default().content(TextContent::Literal { text: text.into() })
    }

    pub fn translate(translation_key: impl Into<Cow<'static, str>>, placeholders: Vec<Text>) -> Self {
        Text::default().content(TextContent::Translatable {
            key: translation_key.into(),
            placeholders,
        })
    }

    pub fn keybind(keybind: impl Into<Cow<'static, str>>) -> Self {
        Text::default().content(TextContent::Keybind {
            keybind: keybind.into(),
        })
    }

    /// Sets color of the text
    pub fn color(mut self, color: TextColor) -> Self {
        self.style.color = Some(color);
        self
    }

    /// Clears the `color` of the text. Color of the parent will be used
    pub fn reset_color(mut self) -> Self {
        self.style.color = None;
        self
    }

    /// Makes the text `bold`
    pub fn bold(mut self) -> Self {
        self.style.bold = Some(true);
        self
    }

    /// Makes the text not `bold`
    pub fn not_bold(mut self) -> Self {
        self.style.bold = Some(false);
        self
    }

    /// Clears the `bold` property of the text. Property of the parent will be used
    pub fn clear_bold(mut self) -> Self {
        self.style.bold = None;
        self
    }

    /// Makes the text `italic`
    pub fn italic(mut self) -> Self {
        self.style.italic = Some(true);
        self
    }

    /// Makes the text not `italic`
    pub fn not_italic(mut self) -> Self {
        self.style.italic = Some(false);
        self
    }

    /// Clears the `italic` property of the text. Property of the parent will be used
    pub fn clear_italic(mut self) -> Self {
        self.style.italic = None;
        self
    }

    /// Makes the text `underlined`
    pub fn underlined(mut self) -> Self {
        self.style.underlined = Some(true);
        self
    }

    /// Makes the text not `underlined`
    pub fn not_underlined(mut self) -> Self {
        self.style.underlined = Some(false);
        self
    }

    /// Clears the `underlined` property of the text. Property of the parent will be used
    pub fn clear_underlined(mut self) -> Self {
        self.style.underlined = None;
        self
    }

    /// Makes the text `strikethrough`
    pub fn strikethrough(mut self) -> Self {
        self.style.underlined = Some(true);
        self
    }

    /// Makes the text not `strikethrough`
    pub fn not_strikethrough(mut self) -> Self {
        self.style.underlined = Some(false);
        self
    }

    /// Clears the `strikethrough` property of the text. Property of the parent will be used
    pub fn clear_strikethrough(mut self) -> Self {
        self.style.strikethrough = None;
        self
    }

    /// When enabled, the client displays quickly changing random characters.
    /// Note: The client has access to the original content, so this is purely visual.
    pub fn obfuscated(mut self) -> Self {
        self.style.obfuscated = Some(true);
        self
    }

    /// Makes the text not `obfuscated`
    pub fn not_obfuscated(mut self) -> Self {
        self.style.obfuscated = Some(false);
        self
    }

    /// Clears the `obfuscated` property of the text. Property of the parent will be used
    pub fn clear_obfuscated(mut self) -> Self {
        self.style.obfuscated = None;
        self
    }

    pub fn font(mut self, font: Option<NamespacedKey>) -> Self {
        self.style.font = font;
        self
    }

    /// Clears the `font` property of the text. Property of the parent will be used
    pub fn clear_font(mut self) -> Self {
        self.style.font = None;
        self
    }

    /// On mouse hover, shows the given text in a tooltip.
    pub fn click_event(mut self, click_event: Option<ClickEvent>) -> Self {
        self.style.click_event = click_event;
        self
    }

    /// Clears the `click_event` property of the text. Property of the parent will be used
    pub fn clear_click_event(mut self) -> Self {
        self.style.click_event = None;
        self
    }

    pub fn hover_event(mut self, hover_event: Option<HoverEvent>) -> Self {
        self.style.hover_event = hover_event;
        self
    }

    pub fn clear_hover_event(mut self) -> Self {
        self.style.hover_event = None;
        self
    }

    pub fn add_child(mut self, children: Text) -> Self {
        self.children.push(children);
        self
    }

    pub fn add_children(mut self, children: &[Text]) -> Self {
        self.children.extend_from_slice(children);
        self
    }

    pub fn style(mut self, style: TextStyle) -> Self {
        self.style = style;
        self
    }

    pub fn content(mut self, content: TextContent) -> Self {
        self.content = content;
        self
    }

    pub fn children(mut self, children: Vec<Text>) -> Self {
        self.children = children;
        self
    }

    pub fn clear_style(self) -> Self {
        self.style(TextStyle::default())
    }
}

impl From<String> for Text {
    fn from(value: String) -> Self {
        Text::literal(value)
    }
}

impl From<&'static str> for Text {
    fn from(value: &'static str) -> Self {
        Text::literal(value)
    }
}

impl FromStr for Text {
    type Err = TextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Text::literal(value.to_string()))
    }
}
