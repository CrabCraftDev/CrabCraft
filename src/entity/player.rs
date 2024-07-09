use evenio::{component::Component, world::World};

use crate::network::connection::NetworkConnection;

pub mod network;

#[derive(Component)]
pub struct Player;

impl Player {
    pub fn spawn(world: &mut World, connection: NetworkConnection) {
        let entity = world.spawn();
        world.insert(entity, Player);
        world.insert(entity, connection);
    }
}
