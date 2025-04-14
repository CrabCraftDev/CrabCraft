use crate::events::asynchronous::AsyncEvent;
use crate::utils::errors::Error;
use crab_macros::AsyncEvent;
use crab_text::formats::TextFormat;
use crab_text::formats::terminal::ColoredTerminalFormat;
use crab_text::text::Text;
use crab_text::text::color::TextColor;
use evenio::event::TargetedEvent;
use std::fmt::{Display, Formatter};

/// Fired after a player left the server
///
/// Based on Bukkit's [PlayerQuitEvent](https://hub.spigotmc.org/javadocs/bukkit/org/bukkit/event/player/PlayerQuitEvent.html)
#[derive(TargetedEvent, AsyncEvent, Debug)]
pub struct PlayerQuitEvent {
    pub quit_reason: PlayerQuitReason,
}

impl PlayerQuitEvent {
    pub fn new(quit_reason: PlayerQuitReason) -> Self {
        PlayerQuitEvent { quit_reason }
    }
}

#[derive(Debug)]
pub enum PlayerQuitReason {
    Disconnected,
    Kicked(Box<Text>),
    TimedOut,
}

impl Display for PlayerQuitReason {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayerQuitReason::Disconnected => formatter.write_str("Disconnected"),
            PlayerQuitReason::Kicked(reason) => {
                let reason = reason.clone().color(TextColor::Gold);
                match ColoredTerminalFormat.write(&reason) {
                    Ok(colored_text) => formatter.write_str(&colored_text),
                    Err(_) => formatter.write_str(&format!("{:?}", reason)),
                }
            }
            PlayerQuitReason::TimedOut => formatter.write_str("Timed out"),
        }
    }
}
