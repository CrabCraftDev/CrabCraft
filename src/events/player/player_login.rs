use crate::events::asynchronous::AsyncEvent;
use crate::network::connection::NetworkConnection;
use crate::utils::errors::Error;
use crab_macros::AsyncEvent;
use evenio::event::GlobalEvent;
use uuid::Uuid;

/// Fired when player finishes login phase and player should be spawned
#[derive(Debug, Clone, GlobalEvent, AsyncEvent)]
pub struct PlayerLoginEvent {
    pub connection: NetworkConnection,
    username: String,
    uuid: Uuid,
    pub status: PlayerLoginStatus,
}

impl PlayerLoginEvent {
    pub fn new(username: String, uuid: Uuid, connection: NetworkConnection) -> Self {
        PlayerLoginEvent {
            connection,
            username,
            uuid,
            status: PlayerLoginStatus::Allowed,
        }
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PlayerLoginStatus {
    Allowed,
    Disallowed(String),
}
