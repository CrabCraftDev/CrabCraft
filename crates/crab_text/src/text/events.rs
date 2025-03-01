use crate::text::Text;
use crate::utils::namespaced_key::NamespacedKey;
use crate::utils::serde::is_one;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Action that happens when player clicks on text component
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "action", content = "value", rename_all = "snake_case")]
pub enum ClickEvent {
    /// Opens a URL in default browser
    OpenUrl(Cow<'static, str>),
    /// Sends message/command in chat as player
    RunCommand(Cow<'static, str>),
    /// Sets content of chat box to this value (not necessarily a command)
    SuggestCommand(Cow<'static, str>),
    /// Only used in books
    ChangePage(i32),
    /// Copies the given text to clipboard
    CopyToClipboard(Cow<'static, str>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "action", content = "contents", rename_all = "snake_case")]
pub enum HoverEvent {
    ShowText(Box<Text>),
    ShowItem {
        id: NamespacedKey,
        #[serde(skip_serializing_if = "is_one")]
        count: i32,
        // Note: This should be sNBT, but CrabCraft doesn't support it yet
        #[serde(skip_serializing_if = "Option::is_none")]
        components: Option<Cow<'static, str>>,
    },
    ShowEntity {
        #[serde(rename = "type")]
        entity_type: NamespacedKey,
        id: Cow<'static, str>,
        name: Option<Box<Text>>,
    },
}
