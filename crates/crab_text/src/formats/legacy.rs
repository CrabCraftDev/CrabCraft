use crate::error::TextError;
use crate::formats::TextFormat;
use crate::text::color::TextColor;
use crate::text::content::TextContent;
use crate::text::style::TextStyle;
use crate::text::Text;

/// Support for parsing legacy §-based formatting system.
/// For more information read [Formatting Codes](https://minecraft.wiki/w/Formatting_codes)
#[derive(Debug)]
pub struct LegacyTextFormat {
    character: &'static char,
}

const AMPERSAND_CHARACTER: char = '&';
const SECTION_CHARACTER: char = '§';

impl LegacyTextFormat {
    pub fn ampersand() -> LegacyTextFormat {
        LegacyTextFormat {
            character: &AMPERSAND_CHARACTER,
        }
    }

    pub fn section() -> LegacyTextFormat {
        LegacyTextFormat {
            character: &SECTION_CHARACTER,
        }
    }

    fn write_style(&self, style: &TextStyle, result: &mut String) -> Result<(), TextError> {
        if let Some(color) = &style.color {
            let symbol = format!("{:X}", color.get_type_id()?).to_ascii_lowercase();
            self.add_format_code(result, symbol.chars().next().unwrap_or_default());
        }

        if style.bold.unwrap_or_default() {
            self.add_format_code(result, 'l');
        }
        if style.strikethrough.unwrap_or_default() {
            self.add_format_code(result, 'm');
        }
        if style.underlined.unwrap_or_default() {
            self.add_format_code(result, 'n');
        }
        if style.italic.unwrap_or_default() {
            self.add_format_code(result, 'o');
        }
        if style.obfuscated.unwrap_or_default() {
            self.add_format_code(result, 'k');
        }

        Ok(())
    }

    fn write_text_recursive(
        &self,
        text: &Text,
        result: &mut String,
        parent_style: &TextStyle,
    ) -> Result<(), TextError> {
        self.write_style(&text.style, result)?;

        match &text.content {
            TextContent::Literal { text } => result.push_str(text),
            _ => return Err(TextError::LegacyFormatSupportsOnlyLiteral),
        }

        for child in &text.children {
            self.write_text_recursive(child, result, &text.style)?;
        }

        if !result.ends_with(&format!("{}r", self.character)) {
            self.add_format_code(result, 'r');
        }
        self.write_style(parent_style, result)?;

        Ok(())
    }

    fn add_format_code(&self, result: &mut String, code: char) {
        result.push(*self.character);
        result.push(code);
    }
}

impl TextFormat for LegacyTextFormat {
    type TargetFormatConversion = String;

    fn parse(&self, input: &Self::TargetFormatConversion) -> Result<Text, TextError> {
        let mut elements: Vec<Text> = Vec::new();

        let mut text = String::new();
        let mut style = TextStyle::default();

        let mut chars = input.chars();
        while let Some(char) = chars.next() {
            if char.eq_ignore_ascii_case(self.character) {
                if !text.is_empty() {
                    let text = std::mem::take(&mut text);
                    elements.push(Text::literal(text).style(style.clone()));
                }

                let Some(modifier) = chars.next() else { continue };

                match modifier {
                    '0'..='9' | 'a'..='f' => {
                        if let Some(color) = TextColor::from_repr(modifier.to_digit(16).unwrap() as u8) {
                            style = TextStyle::default();
                            style.color = Some(color);
                        }
                    }
                    'k' => style.obfuscated = Some(true),
                    'l' => style.bold = Some(true),
                    'm' => style.strikethrough = Some(true),
                    'n' => style.underlined = Some(true),
                    'o' => style.italic = Some(true),
                    'r' => style = TextStyle::default(),
                    _ => {}
                }
            } else {
                text += &char.to_string();
            }
        }

        elements.push(Text::literal(text).style(style));
        Ok(Text::literal("").children(elements))
    }

    fn write(&self, text: &Text) -> Result<Self::TargetFormatConversion, TextError> {
        let mut result = String::new();
        self.write_text_recursive(text, &mut result, &TextStyle::default())?;
        Ok(result)
    }
}
