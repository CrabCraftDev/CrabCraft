use crab_text::error::TextError;
use crab_text::text::color::TextColor;
use nu_ansi_term::Color;

#[test]
fn test_get_type_id() {
    assert_eq!(TextColor::Black.get_type_id().unwrap(), 0);
    assert_eq!(TextColor::DarkBlue.get_type_id().unwrap(), 1);
    assert!(matches!(
        TextColor::Rgb(255, 0, 0).get_type_id(),
        Err(TextError::LegacyFormatDoesNotSupportRgb)
    ));
}

#[test]
fn test_terminal_color() {
    assert_eq!(TextColor::Black.terminal_color(), Color::Black);
    assert_eq!(TextColor::Rgb(255, 0, 0).terminal_color(), Color::Rgb(255, 0, 0));
}

#[test]
fn test_serialize_standard_colors() {
    let serialized = serde_json::to_string(&TextColor::DarkBlue).unwrap();
    assert_eq!(serialized, "\"dark_blue\"");
}

#[test]
fn test_serialize_rgb_color() {
    let serialized = serde_json::to_string(&TextColor::Rgb(255, 165, 0)).unwrap();
    assert_eq!(serialized, "\"#FFA500\"");
}

#[test]
fn test_deserialize_standard_colors() {
    let deserialized: TextColor = serde_json::from_str("\"dark_blue\"").unwrap();
    assert_eq!(deserialized, TextColor::DarkBlue);
}

#[test]
fn test_deserialize_rgb_color() {
    let deserialized: TextColor = serde_json::from_str("\"#FFA500\"").unwrap();
    assert_eq!(deserialized, TextColor::Rgb(255, 165, 0));
}

#[test]
fn test_deserialize_invalid_rgb_color() {
    let result: Result<TextColor, _> = serde_json::from_str("\"#GGGGGG\"");
    assert!(result.is_err());
}

#[test]
fn test_deserialize_unknown_color() {
    let result: Result<TextColor, _> = serde_json::from_str("\"unknown_color\"");
    assert!(result.is_err());
}

#[test]
fn test_rgb_format_error() {
    let result: Result<TextColor, _> = serde_json::from_str("\"#12345\"");
    assert!(result.is_err());
}
