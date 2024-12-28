use crate::events::asynchronous::AsyncEvent;
use crate::utils::errors::Error;
use crab_macros::AsyncEvent;
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
    Kicked(String),
    TimedOut,
}

impl Display for PlayerQuitReason {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayerQuitReason::Disconnected => formatter.write_str("Disconnected"),
            PlayerQuitReason::Kicked(reason) => formatter.write_str(reason),
            PlayerQuitReason::TimedOut => formatter.write_str("Timed out"),
        }
    }
}
