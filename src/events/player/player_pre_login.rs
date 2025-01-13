use crate::events::asynchronous::AsyncEvent;
use crate::utils::errors::Error;
use crab_macros::AsyncEvent;
use crab_text::text::Text;
use evenio::event::GlobalEvent;
use std::net::IpAddr;
use uuid::Uuid;

/// Fired just before a player finishes login phase
/// Used for simple checks like is player banned, is player whitelisted
/// Checks for duplicate uuid / username is made in PlayerLoginEvent handler
///
/// Based on Bukkit's [AsyncPlayerPreLoginEvent](https://hub.spigotmc.org/javadocs/bukkit/org/bukkit/event/player/AsyncPlayerPreLoginEvent.html)
#[allow(unused)]
#[derive(GlobalEvent, AsyncEvent, Debug)]
pub struct PlayerPreLoginEvent {
    username: String,
    ip_address: IpAddr,
    uuid: Uuid,
    pub status: PlayerPreLoginStatus,
}

impl PlayerPreLoginEvent {
    pub fn new(username: String, ip_address: IpAddr, uuid: Uuid) -> Self {
        PlayerPreLoginEvent {
            username,
            ip_address,
            uuid,
            status: PlayerPreLoginStatus::Allowed,
        }
    }

    #[allow(unused)]
    pub fn username(&self) -> &str {
        &self.username
    }

    #[allow(unused)]
    pub fn ip_address(&self) -> &IpAddr {
        &self.ip_address
    }

    #[allow(unused)]
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }
}

#[allow(unused)]
#[derive(Debug)]
pub enum PlayerPreLoginStatus {
    Allowed,
    KickBanned,
    KickFull,
    KickWhitelist,
    KickOther(Text),
}
