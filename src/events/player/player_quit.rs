use crate::events::asynchronous::AsyncEvent;
use crate::utils::errors::Error;
use crab_macros::AsyncEvent;
use evenio::event::TargetedEvent;

/// Fired after a player left the server
///
/// Based on Bukkit's [PlayerQuitEvent](https://hub.spigotmc.org/javadocs/bukkit/org/bukkit/event/player/PlayerQuitEvent.html)
#[derive(TargetedEvent, AsyncEvent, Debug)]
pub struct PlayerQuitEvent {
    #[allow(unused)]
    quit_reason: PlayerQuitReason,
}

impl PlayerQuitEvent {
    pub fn new(quit_reason: PlayerQuitReason) -> Self {
        PlayerQuitEvent { quit_reason }
    }
}

#[derive(Debug)]
pub enum PlayerQuitReason {
    Disconnected,
    Kicked,
    TimedOut,
}
