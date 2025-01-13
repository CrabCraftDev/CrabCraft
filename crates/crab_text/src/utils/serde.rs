use crate::text::color::TextColor;
use crate::text::Text;

pub fn is_false(val: &bool) -> bool {
    !val
}

pub fn is_one(val: &i32) -> bool {
    *val == 1
}

pub fn default_separator() -> Option<Box<Text>> {
    Some(Box::new(Text::literal(", ")))
}

pub fn default_colored_separator() -> Option<Box<Text>> {
    Some(Box::new(Text::literal(", ").color(TextColor::Gray)))
}
