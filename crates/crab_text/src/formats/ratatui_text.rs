use crate::error::TextError;
use crate::formats::TextFormat;
use crate::text::Text;
use crate::text::color::TextColor;
use crate::text::content::TextContent;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text as RatatuiText};

pub struct RatatuiTextFormat;

impl RatatuiTextFormat {
    fn inner(text: &Text, style: &Style, buffer: &mut Vec<Vec<Span<'static>>>) -> Result<(), TextError> {
        let text_string = match &text.content {
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
                    format: "RatatuiTextFormat".to_string(),
                    message: format!("writing {:?}", text.content),
                });
            }
        };

        let mut style = *style;

        if let Some(color) = text.style.color {
            style = style.fg(color.into());
        }

        style = match text.style.bold {
            Some(true) => style.add_modifier(Modifier::BOLD),
            Some(false) => style.remove_modifier(Modifier::BOLD),
            None => style,
        };

        style = match text.style.italic {
            Some(true) => style.add_modifier(Modifier::ITALIC),
            Some(false) => style.remove_modifier(Modifier::ITALIC),
            None => style,
        };

        style = match text.style.underlined {
            Some(true) => style.add_modifier(Modifier::UNDERLINED),
            Some(false) => style.remove_modifier(Modifier::UNDERLINED),
            None => style,
        };

        style = match text.style.strikethrough {
            Some(true) => style.add_modifier(Modifier::CROSSED_OUT),
            Some(false) => style.remove_modifier(Modifier::CROSSED_OUT),
            None => style,
        };

        let mut lines = text_string.split('\n').map(ToOwned::to_owned);

        if let Some(line) = lines.next() {
            let span = Span::styled(line, style);
            if let Some(last_line) = buffer.last_mut() {
                last_line.push(span);
            } else {
                buffer.push(vec![span]);
            }
        }

        for line in lines {
            if line.is_empty() {
                buffer.push(vec![]);
            } else {
                let span = Span::styled(line, style);
                buffer.push(vec![span]);
            }
        }

        for child in &text.children {
            Self::inner(child, &style, buffer)?;
        }

        Ok(())
    }
}

impl TextFormat for RatatuiTextFormat {
    type TargetFormatConversion = RatatuiText<'static>;

    fn parse(&self, _input: &RatatuiText) -> Result<Text, TextError> {
        Err(TextError::UnsupportedFormatOperation {
            format: "RatatuiTextFormat".to_string(),
            message: "parsing".to_string(),
        })
    }

    fn write(&self, text: &Text) -> Result<RatatuiText<'static>, TextError> {
        let mut buffer = Vec::new();
        Self::inner(text, &Style::new(), &mut buffer)?;
        let lines: Vec<Line> = buffer.into_iter().map(Line::from).collect();
        Ok(RatatuiText::from(lines))
    }
}

impl From<TextColor> for Color {
    fn from(value: TextColor) -> Self {
        match value {
            TextColor::Black => Color::Black,
            TextColor::DarkBlue => Color::Blue,
            TextColor::DarkGreen => Color::Green,
            TextColor::DarkCyan => Color::Cyan,
            TextColor::DarkRed => Color::Red,
            TextColor::Purple => Color::Magenta,
            TextColor::Gold => Color::Yellow,
            TextColor::Gray => Color::Gray,
            TextColor::DarkGray => Color::DarkGray,
            TextColor::Blue => Color::LightBlue,
            TextColor::Green => Color::LightGreen,
            TextColor::Cyan => Color::LightCyan,
            TextColor::Red => Color::LightRed,
            TextColor::Pink => Color::LightMagenta,
            TextColor::Yellow => Color::LightYellow,
            TextColor::White => Color::White,
            TextColor::Rgb(r, g, b) => Color::Rgb(r, g, b),
            TextColor::Reset => Color::Reset,
        }
    }
}

impl From<&TextColor> for Color {
    fn from(value: &TextColor) -> Self {
        (*value).into()
    }
}
