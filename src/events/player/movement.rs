use crab_protocol::types::position::{Position, Rotation};
use evenio::event::TargetedEvent;

#[derive(TargetedEvent)]
pub struct PlayerMoveEvent {
    pub position: Position,
    pub rotation: Rotation,
}

#[derive(TargetedEvent)]
pub struct PlayerTeleportEvent {
    pub position: Position,
    pub rotation: Rotation,
}
