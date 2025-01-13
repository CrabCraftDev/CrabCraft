use crab_text::formats::terminal::ColoredTerminalFormat;
use crab_text::formats::TextFormat;
use crab_text::text::color::TextColor;
use crab_text::text::Text;

#[test]
fn test_terminal() {
    const CORRECT_COLORED_TEXT: &str =
        "\u{1b}[92mCrabCraft \u{1b}[0m\u{1b}[3;4;35msupports\u{1b}[0m\u{1b}[1;92m console formatting!\u{1b}[0m";

    let text = Text::literal("CrabCraft ")
        .color(TextColor::Green)
        .add_child(
            Text::literal("supports")
                .color(TextColor::Purple)
                .italic()
                .strikethrough(),
        )
        .add_child(Text::literal(" console formatting!").bold());

    let output = ColoredTerminalFormat.write(&text).unwrap();

    assert_eq!(CORRECT_COLORED_TEXT, output, "Text {output} isn't colored properly!")
}
