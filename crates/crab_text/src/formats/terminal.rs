use crate::error::TextError;
use crate::formats::TextFormat;
use crate::text::content::TextContent;
use crate::text::Text;
use nu_ansi_term::Style;

pub struct ColoredTerminalFormat;

impl ColoredTerminalFormat {
    fn inner(text: &Text, default_style: &Style) -> Result<String, TextError> {
        let mut text_string = match &text.content {
            TextContent::Literal { text } => text.to_string(),
            TextContent::Keybind { keybind } => format!("[{}]", keybind),
            TextContent::Translatable { key, placeholders } => {
                if placeholders.is_empty() {
                    format!("[{}]", key)
                } else {
                    format!("[{} ({:?})]", key, placeholders)
                }
            }
            _ => {
                return Err(TextError::UnsupportedFormatOperation {
                    format: "ColoredTerminalFormat".to_string(),
                    message: format!("writing {:?}", text.content),
                })
            }
        };

        let mut style = *default_style;

        if let Some(color) = text.style.color {
            style.foreground = Some(color.terminal_color());
        }

        if let Some(bold) = text.style.bold {
            style.is_bold = bold;
        }

        if let Some(italic) = text.style.italic {
            style.is_italic = italic;
        }

        if let Some(underlined) = text.style.underlined {
            style.is_underline = underlined;
        }

        if let Some(strikethrough) = text.style.strikethrough {
            style.is_strikethrough = strikethrough;
        }

        text_string = style.paint(text_string).to_string();

        for child in &text.children {
            let child_string = Self::inner(child, &style)?;
            text_string.push_str(&child_string);
        }

        Ok(text_string)
    }
}

impl TextFormat for ColoredTerminalFormat {
    type TargetFormatConversion = String;

    fn parse(&self, _input: &String) -> Result<Text, TextError> {
        Err(TextError::UnsupportedFormatOperation {
            format: "ColoredTerminalFormat".to_string(),
            message: "parsing".to_string(),
        })
    }

    fn write(&self, text: &Text) -> Result<String, TextError> {
        let style = Style {
            is_bold: text.style.bold.unwrap_or_default(),
            is_italic: text.style.italic.unwrap_or_default(),
            is_underline: text.style.underlined.unwrap_or_default(),
            is_strikethrough: text.style.strikethrough.unwrap_or_default(),
            ..Default::default()
        };
        Self::inner(text, &style)
    }
}
