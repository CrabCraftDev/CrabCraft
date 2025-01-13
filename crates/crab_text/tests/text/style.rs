use crab_nbt::serde::ser::to_bytes_unnamed;
use crab_nbt::{Nbt, NbtCompound};
use crab_text::text::color::TextColor;
use crab_text::text::style::TextStyle;
use crab_text::utils::namespaced_key::NamespacedKey;
use std::default::Default;

#[test]
fn test_is_empty() {
    let style = TextStyle::default();
    assert!(style.is_empty());

    let style_with_color = TextStyle {
        color: Some(TextColor::Rgb(10, 10, 10)),
        ..Default::default()
    };
    assert!(!style_with_color.is_empty());
}

#[test]
fn test_serialization() {
    let style = TextStyle {
        color: Some(TextColor::Rgb(5, 5, 5)),
        bold: Some(true),
        italic: None,
        underlined: Some(false),
        strikethrough: Some(true),
        obfuscated: None,
        font: Some(NamespacedKey::new("minecraft", "default").unwrap()),
        ..Default::default()
    };

    let mut nbt = to_bytes_unnamed(&style).unwrap();

    let expected = Nbt::new(
        "".to_string(),
        NbtCompound::from_iter([
            ("color".to_owned(), "#050505".into()),
            ("bold".to_owned(), 1i8.into()),
            ("underlined".to_owned(), 0i8.into()),
            ("strikethrough".to_owned(), 1i8.into()),
            ("font".to_owned(), "minecraft:default".into()),
        ]),
    );

    assert_eq!(Nbt::read_unnamed(&mut nbt).unwrap(), expected);
}
