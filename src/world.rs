use crate::entity::player;
use evenio::prelude::{GlobalEvent, World};

/// Creates a new World with all the default systems (handlers)
pub fn new() -> World {
    let mut world = World::new();

    // Player login & other connection stuff
    world.add_handler(player::network::connection::on_disconnect);
    world.add_handler(player::network::configuration_handler::configuration_packets);
    world.add_handler(player::network::login_handler::login_packets);
    world.add_handler(player::network::play_handler::on_world_join);

    // Connection lifecycle
    world.add_handler(player::network::connection::receive_packets);
    world.add_handler(player::network::connection::keep_alive_tick);
    world.add_handler(player::network::connection::on_keep_alive_response);
    world.add_handler(player::network::connection::kick_on_shutdown);

    world
}

#[derive(GlobalEvent)]
pub struct WorldTick;

#[derive(GlobalEvent)]
pub struct Shutdown;
