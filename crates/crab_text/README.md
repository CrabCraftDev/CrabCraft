# `crab_text`

Crate based on [Adventure](https://github.com/KyoriPowered/adventure), that adds support for Minecraft's text component.
For more details on text formatting, see [Minecraft Formatting Codes](https://minecraft.wiki/w/Formatting_codes).

## Documentation

The core element of this crate is the `Text` struct, which provides methods for styling, applying colors, adding
interactions, and more.

```rust
use crab_text::text::color::TextColor;
use crab_text::text::Text;
use crab_text::formats::json::JsonTextFormat;
use crab_text::formats::TextFormat;

let text = Text::literal("hi")
    .bold()
    .color(TextColor::Red);

// {"text":"hi","color":"red","bold":true}
let json = JsonTextFormat.write(&text).unwrap();
```

## Text Formats

Supported Formats:

- Terminal: Formats text with support for terminal colors.
- Ratatui: Converts text for use with the `ratatui` crate.
- NBT: Used in networking, packets
- JSON: Utilized for server list pings and command

Planned but not yet implemented:

- Minimessage
- Legacy

## Legacy format

Minecraft also supports a legacy formatting system using the § symbol followed by a character or number.
This legacy format is processed after standard text components.

Note: The legacy format is **deprecated** and may not work reliably everywhere.
If you need to support it, consider using `LegacyFormat` for better compatibility.