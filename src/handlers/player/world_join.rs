use crate::components::name::Name;
use crate::events::player::player_join_world::PlayerJoinWorldEvent;
use crate::network::connection::NetworkConnection;
use crate::world::chunk_loader::{VIEW_DISTANCE, WORLD_LOADER};
use crab_protocol::packets::play::{
    GameEventS2CPacket, GameJoinS2CPacket, PlayerPositionS2CPacket, SetCenterChunkS2CPacket,
    SetDefaultSpawnPositionS2CPacket,
};
use crab_protocol::types::block_position::BlockPosition;
use crab_protocol::types::game_mode::GameMode;
use crab_protocol::types::namespaced_key::NamespacedKey;
use crab_protocol::types::player_spawn_information::PlayerSpawnInformation;
use crab_protocol::types::position::{Position, Rotation, Vec3Double};
use evenio::entity::EntityId;
use evenio::event::Receiver;
use tracing::info;

#[allow(clippy::type_complexity)]
pub fn on_world_join(
    receiver: Receiver<PlayerJoinWorldEvent, (EntityId, &NetworkConnection, &Name, &Position, &Rotation)>,
) {
    let (entity, connection, name, position, rotation) = receiver.query;

    connection.send_packet(GameJoinS2CPacket {
        entity_id: 0,
        is_hardcore: false,
        dimension_names: vec![NamespacedKey::minecraft("overworld").unwrap()],
        max_players: 100,
        view_distance: VIEW_DISTANCE as i32,
        simulation_distance: 20,
        reduced_debug_info: false,
        enable_respawn_screen: true,
        do_limited_crafting: false,
        spawn_information: PlayerSpawnInformation {
            dimension_type: 0,
            dimension_name: NamespacedKey::minecraft("overworld").unwrap(),
            hashed_seed: 99887766,
            game_mode: GameMode::Creative,
            previous_game_mode: None,
            is_debug_world: false,
            is_flat_world: true,
            last_death_location: None,
            portal_cooldown: 20,
            sea_level: 64,
        },
        enforces_secure_chat: false,
    });

    // TODO: Temporary implementation of chunk sending
    connection.send_packet(SetDefaultSpawnPositionS2CPacket {
        location: BlockPosition { x: 0, y: 0, z: 0 },
        angle: 0.0,
    });

    connection.send_packet(GameEventS2CPacket { event: 13, value: 0.0 });

    connection.send_packet(SetCenterChunkS2CPacket { x: 0, z: 0 });
    
    let chunk_loader = &mut WORLD_LOADER
        .chunk_loaders
        .get_mut(&NamespacedKey::minecraft("overworld").unwrap())
        .unwrap();
    chunk_loader.add_chunk_ticket(entity, (position.0.x / 16.) as i32, (position.0.z / 16.) as i32);

    connection.send_packet(PlayerPositionS2CPacket {
        teleport_id: 0,
        position: position.clone(),
        delta_movement: Vec3Double::default(),
        rotation: *rotation,
        position_flags: Default::default(),
    });

    info!("{name} joined!");
}
