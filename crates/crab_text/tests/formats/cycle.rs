use crab_text::formats::legacy::LegacyTextFormat;
use crab_text::formats::nbt::NbtTextFormat;
use crab_text::formats::terminal::ColoredTerminalFormat;
use crab_text::formats::TextFormat;
use crab_text::text::color::TextColor;
use crab_text::text::Text;

#[test]
fn test_cycle() {
    let text = Text::literal("CrabCraft ")
        .color(TextColor::Green)
        .add_child(
            Text::literal("supports")
                .color(TextColor::Purple)
                .italic()
                .strikethrough(),
        )
        .add_child(Text::literal(" text formatting!").bold());
    let colored_original_text = ColoredTerminalFormat.write(&text).unwrap();

    let text = NbtTextFormat.parse(&NbtTextFormat.write(&text).unwrap()).unwrap();
    let text = LegacyTextFormat::section()
        .parse(&LegacyTextFormat::section().write(&text).unwrap())
        .unwrap();
    let colored_text = ColoredTerminalFormat.write(&text).unwrap();

    assert_eq!(colored_original_text, colored_text)
}
