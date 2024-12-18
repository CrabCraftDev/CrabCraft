use crate::components::name::Name;
use crate::events::player::player_join_world::PlayerJoinWorldEvent;
use crate::network::connection::NetworkConnection;
use crate::world::worlds_storage::WORLDS_STORAGE;
use crate::world::OVERWORLD_IDENTIFIER;
use bytes::{Bytes, BytesMut};
use crab_nbt::nbt;
use crab_protocol::packets::play::{
    ChunkBatchFinishedS2CPacket, ChunkBatchStartS2CPacket, ChunkDataAndUpdateLightS2CPacket, GameEventS2CPacket,
    GameJoinS2CPacket, LightData, PlayerPositionS2CPacket, SetCenterChunkS2CPacket, SetDefaultSpawnPositionS2CPacket,
};
use crab_protocol::types::bit_set::BitSet;
use crab_protocol::types::block_position::BlockPosition;
use crab_protocol::types::game_mode::GameMode;
use crab_protocol::types::player_spawn_information::PlayerSpawnInformation;
use crab_protocol::types::position::{Position, Rotation, Vec3Double};
use crab_world::chunks::chunk::CHUNK_SECTIONS_PER_WORLD_HEIGHT;
use crab_world::chunks::section::AMOUNT_OF_BLOCKS_IN_CHUNK_SECTION;
use evenio::event::Receiver;
use tracing::info;

#[allow(clippy::type_complexity)]
pub fn on_world_join(receiver: Receiver<PlayerJoinWorldEvent, (&NetworkConnection, &Name, &Position, &Rotation)>) {
    let (connection, name, position, rotation) = receiver.query;

    connection.send_packet(GameJoinS2CPacket {
        entity_id: 0,
        is_hardcore: false,
        dimension_names: vec![OVERWORLD_IDENTIFIER.clone()],
        max_players: 100,
        view_distance: 16,
        simulation_distance: 20,
        reduced_debug_info: false,
        enable_respawn_screen: true,
        do_limited_crafting: false,
        spawn_information: PlayerSpawnInformation {
            dimension_type: 0,
            dimension_name: OVERWORLD_IDENTIFIER.clone(),
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

    connection.send_packet(ChunkBatchStartS2CPacket);

    let dimension = WORLDS_STORAGE.worlds.get_mut(&OVERWORLD_IDENTIFIER).unwrap();

    let mut number_of_sent_chunks = 0;
    for x in 0..17 {
        for z in 0..17 {
            number_of_sent_chunks += 1;

            let mut bytes = BytesMut::new();
            dimension.serialize_chunk(&mut bytes, x, z).unwrap();

            let heightmap = nbt!("", {
                "MOTION_BLOCKING": [L;],
                "WORLD_SURFACE": [L;],
            });

            // Temporary implementation of light
            let sky_light_mask = BitSet::new(vec![true; CHUNK_SECTIONS_PER_WORLD_HEIGHT + 2]);
            let chunk_section_sky_light_array = Bytes::from(vec![u8::MAX; AMOUNT_OF_BLOCKS_IN_CHUNK_SECTION / 2]);
            let chunk_sky_light_array = vec![chunk_section_sky_light_array; CHUNK_SECTIONS_PER_WORLD_HEIGHT + 2];

            connection.send_packet(ChunkDataAndUpdateLightS2CPacket {
                chunk_x: x,
                chunk_z: z,
                heightmaps: heightmap,
                data: bytes.freeze(),
                block_entities: vec![],
                light_data: LightData {
                    sky_light_mask,
                    block_light_mask: Default::default(),
                    empty_sky_light_mask: Default::default(),
                    empty_block_light_mask: Default::default(),
                    sky_light_array: chunk_sky_light_array,
                    block_light_array: Default::default(),
                },
            });
        }
    }

    connection.send_packet(ChunkBatchFinishedS2CPacket {
        number_of_chunks: number_of_sent_chunks,
    });

    connection.send_packet(PlayerPositionS2CPacket {
        teleport_id: 0,
        position: position.clone(),
        delta_movement: Vec3Double::default(),
        rotation: *rotation,
        position_flags: Default::default(),
    });

    info!("{name} joined!");
}
