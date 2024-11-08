use crab_protocol::types::position::{Position, Rotation};
use evenio::component::Component;
use evenio::event::TargetedEvent;

pub mod network;
pub mod world;

#[derive(Component)]
pub struct Player;

#[derive(Component, Default)]
pub struct LastTeleport {
    pub id: i32,
    pub confirmed: bool,
}

#[derive(TargetedEvent)]
pub struct TeleportEvent {
    pub position: Position,
    pub rotation: Rotation,
}

#[derive(TargetedEvent)]
pub struct MoveEvent {
    pub position: Position,
    pub rotation: Rotation,
}
