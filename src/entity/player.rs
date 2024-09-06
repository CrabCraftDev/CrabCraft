use evenio::component::Component;
use evenio::event::TargetedEvent;

pub mod network;

#[derive(TargetedEvent)]
pub struct PlayerDisconnected;

#[derive(TargetedEvent)]
pub struct PlayerJoined;

#[derive(Component)]
pub struct KeepAliveStatus {
    pub(crate) keepalive_pending: bool,
    pub(crate) last_keepalive_time: i64,
}
