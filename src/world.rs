use evenio::{event::GlobalEvent, world::World};

use crate::entity::player;

/// Creates a new World with all the defualt systems (handlers)
pub fn new() -> World {
    let mut world = World::new();

    // Player entity
    world.add_handler(player::network::connection::receive_packets);
    world.add_handler(player::network::connection::on_disconnect);
    world.add_handler(player::network::configuration::configuration_packets);
    world.add_handler(player::network::play::on_world_join);

    world
}

#[derive(GlobalEvent)]
pub struct WorldTick;
