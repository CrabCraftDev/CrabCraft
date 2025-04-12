use crate::error::TextError;
use nu_ansi_term::Color as AnsiColor;
use ratatui::style::Color as RatatuiColor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use strum::{Display, EnumString, FromRepr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString, FromRepr)]
#[strum(serialize_all = "snake_case")]
#[repr(u8)]
pub enum TextColor {
    Black,
    DarkBlue,
    DarkGreen,
    #[strum(serialize = "dark_aqua")]
    DarkCyan,
    DarkRed,
    #[strum(serialize = "dark_purple")]
    Purple,
    Gold,
    Gray,
    DarkGray,
    Blue,
    Green,
    #[strum(serialize = "aqua")]
    Cyan,
    Red,
    #[strum(serialize = "light_purple")]
    Pink,
    Yellow,
    White,
    Rgb(u8, u8, u8),
    Reset,
}

impl TextColor {
    pub const fn get_type_id(&self) -> Result<u8, TextError> {
        match self {
            TextColor::Rgb(..) => Err(TextError::LegacyFormatDoesNotSupportRgb),
            // See https://doc.rust-lang.org/reference/items/enumerations.html#pointer-casting
            _ => Ok(unsafe { *(self as *const Self as *const u8) }),
        }
    }

    pub fn to_terminal_color(&self) -> AnsiColor {
        self.into()
    }

    pub fn to_ratatui_color(&self) -> RatatuiColor {
        self.into()
    }
}

impl Serialize for TextColor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            TextColor::Rgb(r, g, b) => serializer.serialize_str(&format!("#{:02X}{:02X}{:02X}", r, g, b)),
            _ => serializer.serialize_str(&self.to_string()),
        }
    }
}

impl<'de> Deserialize<'de> for TextColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if let Some('#') = s.chars().next() {
            if s.len() == 7 {
                let r = u8::from_str_radix(&s[1..3], 16).map_err(serde::de::Error::custom)?;
                let g = u8::from_str_radix(&s[3..5], 16).map_err(serde::de::Error::custom)?;
                let b = u8::from_str_radix(&s[5..7], 16).map_err(serde::de::Error::custom)?;
                return Ok(TextColor::Rgb(r, g, b));
            } else {
                return Err(serde::de::Error::custom("Invalid RGB format"));
            }
        }

        s.parse().map_err(|_| serde::de::Error::custom("Unknown color"))
    }
}
