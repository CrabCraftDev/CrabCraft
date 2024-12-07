use crate::events::asynchronous::AsyncEvent;
use crate::utils::errors::Error;
use crab_macros::AsyncEvent;
use crab_protocol::types::namespaced_key::NamespacedKey;
use evenio::event::GlobalEvent;
use tokio::time::Instant;

#[derive(GlobalEvent, AsyncEvent, Debug)]
pub struct WorldTickEvent {
    pub tick_start: Instant,
}

#[derive(GlobalEvent, Debug)]
pub struct ChunksLoadedEvent {
    pub dimension: NamespacedKey,
    pub chunks: Vec<(i32, i32)>,
}
