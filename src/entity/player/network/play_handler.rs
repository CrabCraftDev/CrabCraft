use crate::{entity::components::name::Name, network::connection::NetworkConnection};
use bytes::{Bytes, BytesMut};
use crab_nbt::nbt;
use crab_protocol::packets::common::{Tag, UpdateTagsS2CPacket};
use crab_protocol::{
    packets::play::{
        ChunkBatchFinishedS2CPacket, ChunkBatchStartS2CPacket, ChunkDataAndUpdateLightS2CPacket, GameEventS2CPacket,
        GameJoinS2CPacket, LightData, SetCenterChunkS2CPacket, SetDefaultSpawnPositionS2CPacket,
        SynchronizePlayerPositionS2CPacket, SynchronizePositionFlags,
    },
    types::{
        bit_set::BitSet, block_position::BlockPosition, game_mode::GameMode, namespaced_key::NamespacedKey,
        position::Position,
    },
};
use crab_world::blocks::{Bedrock, BlockType, Dirt, GrassBlock};
use crab_world::chunks::chunk::{Chunk, CHUNK_SECTIONS_PER_WORLD_HEIGHT};
use crab_world::chunks::section::AMOUNT_OF_BLOCKS_IN_CHUNK_SECTION;
use evenio::event::{Receiver, TargetedEvent};
use log::info;
use std::collections::HashMap;

#[derive(TargetedEvent)]
pub struct PlayerJoinWorldEvent;

pub fn on_world_join(receiver: Receiver<PlayerJoinWorldEvent, (&NetworkConnection, &Name)>) {
    let (connection, name) = receiver.query;

    connection.send_packet(GameJoinS2CPacket {
        entity_id: 0,
        is_hardcore: false,
        dimension_names: vec![NamespacedKey::minecraft("overworld").unwrap()],
        max_players: 100,
        view_distance: 16,
        simulation_distance: 20,
        reduced_debug_info: false,
        enable_respawn_screen: true,
        do_limited_crafting: false,
        dimension_type: 0,
        dimension_name: NamespacedKey::minecraft("overworld").unwrap(),
        hashed_seed: 99887766,
        game_mode: GameMode::Creative,
        previous_game_mode: None.into(),
        is_debug_world: false,
        is_flat_world: true,
        last_death_location: None,
        portal_cooldown: 20,
        enforces_secure_chat: false,
    });

    // TODO: Temporary implementation of chunk sending
    connection.send_packet(SetDefaultSpawnPositionS2CPacket {
        location: BlockPosition { x: 0, y: 0, z: 0 },
        angle: 0.0,
    });

    connection.send_packet(SetCenterChunkS2CPacket { x: 0, z: 0 });

    connection.send_packet(GameEventS2CPacket { event: 13, value: 0.0 });

    connection.send_packet(ChunkBatchStartS2CPacket);

    let mut tags = HashMap::new();
    tags.insert(
        NamespacedKey::minecraft("fluid").unwrap(),
        vec![
            Tag {
                name: NamespacedKey::minecraft("water").unwrap(),
                entries: vec![2, 1],
            },
            Tag {
                name: NamespacedKey::minecraft("lava").unwrap(),
                entries: vec![4, 3],
            },
        ],
    );
    connection.send_packet(UpdateTagsS2CPacket { tags });

    let mut normal_chunk = Chunk::default();
    let mut air_chunk = Chunk::default();

    let grass_section = &mut normal_chunk.sections[0];

    for i in 0..16 {
        for j in 0..16 {
            grass_section
                .set_block(i, 4, j, GrassBlock::default_state_id().unwrap())
                .unwrap();
            grass_section
                .set_block(i, 3, j, Dirt::default_state_id().unwrap())
                .unwrap();
            grass_section
                .set_block(i, 2, j, Dirt::default_state_id().unwrap())
                .unwrap();
            grass_section
                .set_block(i, 1, j, Dirt::default_state_id().unwrap())
                .unwrap();
            grass_section
                .set_block(i, 0, j, Bedrock::default_state_id().unwrap())
                .unwrap();
        }
    }

    let view_distance: i32 = 8;
    for x in -view_distance..view_distance {
        for z in -view_distance..view_distance {
            let mut bytes = BytesMut::new();

            let chunk = if x.abs() < (view_distance - 2) && z.abs() < (view_distance - 2) {
                &mut normal_chunk
            } else {
                &mut air_chunk
            };
            chunk.serialize_to_mc_format(&mut bytes).unwrap();

            let world_surface_height_map = chunk.serialize_world_surface_height_map();
            let motion_blocking_height_map = world_surface_height_map.clone();
            let heightmap = nbt!("", {
                "MOTION_BLOCKING": motion_blocking_height_map,
                "WORLD_SURFACE": world_surface_height_map,
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
        number_of_chunks: (view_distance * 2 + 1).pow(2),
    });

    connection.send_packet(SynchronizePlayerPositionS2CPacket {
        position: Position::new(0.0, -59.0, 0.0),
        yaw: 0.0,
        pitch: 0.0,
        flags: SynchronizePositionFlags::new(),
        teleport_id: 2137,
    });

    info!("{name} joined!");
}
