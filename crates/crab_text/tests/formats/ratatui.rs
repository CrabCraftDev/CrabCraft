use crab_text::formats::TextFormat;
use crab_text::formats::ratatui_text::RatatuiTextFormat;
use crab_text::text::Text;
use crab_text::text::color::TextColor;
use ratatui::{
    style::Stylize,
    text::{Line, Span, Text as RatatuiText},
};

#[test]
fn test_ratatui() {
    let reference_text = RatatuiText::from_iter([
        Line::from_iter([
            Span::from("CrabCraft ").light_green(),
            Span::from("supports").magenta().italic().underlined(),
            Span::from(" Ratatui!").light_green().bold(),
        ]),
        Line::from(Span::from("Also this should be in separate line :-)").light_green()),
    ]);

    let text = Text::literal("CrabCraft ")
        .color(TextColor::Green)
        .add_child(
            Text::literal("supports")
                .color(TextColor::Purple)
                .italic()
                .strikethrough(),
        )
        .add_child(Text::literal(" Ratatui!\n").bold())
        .add_child(Text::literal("Also this should be in separate line :-)"));

    let output = RatatuiTextFormat.write(&text).unwrap();

    assert_eq!(reference_text, output)
}
