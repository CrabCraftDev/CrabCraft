use evenio::component::Component;

pub mod network;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct LastTeleport(pub i32, pub bool);
