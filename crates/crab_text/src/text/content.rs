use crate::text::Text;
use crate::utils::serde::{default_colored_separator, default_separator, is_false};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
#[repr(u8)]
pub enum TextContent {
    /// Normal text, but legacy formatting still applies
    Literal { text: Cow<'static, str> },
    /// Translates text based on player's locale
    Translatable {
        #[serde(rename = "translate")]
        key: Cow<'static, str>,
        /// The translation text may contain any number of the placeholder tokens
        #[serde(default, rename = "with", skip_serializing_if = "Vec::is_empty")]
        placeholders: Vec<Text>,
    },
    /// Displays the name of the button that is currently bound to a certain keybind
    Keybind { keybind: Cow<'static, str> },
    /// Displays a score holder's current score in an objective.
    Score { score: ScoreContent },
    /// Displays the name of one or more entities found by a [selector](https://minecraft.wiki/w/Target_selectors).
    Selector {
        selector: Cow<'static, str>,
        #[serde(default = "default_colored_separator", skip_serializing_if = "Option::is_none")]
        separator: Option<Box<Text>>,
    },
    /// Resolved by the server into a delimited list of all NBT values, for all objects specified by the data source, matching the specified path.
    Nbt {
        #[serde(rename = "nbt")]
        nbt_path: Cow<'static, str>,
        #[serde(default, skip_serializing_if = "is_false")]
        interpret: bool,
        #[serde(default = "default_separator", skip_serializing_if = "Option::is_none")]
        separator: Option<Box<Text>>,
        #[serde(flatten)]
        data_source: NbtContentDataSource,
    },
}

impl Default for TextContent {
    fn default() -> Self {
        Self::Literal { text: "".into() }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ScoreContent {
    name: String,
    objective: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
#[repr(u8)]
pub enum NbtContentDataSource {
    Block(String),
    Entity(String),
    Storage(String),
}
