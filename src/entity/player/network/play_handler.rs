use crate::utils::files::FILES;
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
use crab_registry::REGISTRY;
use crab_world::anvil::region::Region;
use crab_world::chunks::chunk::CHUNK_SECTIONS_PER_WORLD_HEIGHT;
use crab_world::chunks::section::{ChunkSection, AMOUNT_OF_BLOCKS_IN_CHUNK_SECTION};
use evenio::event::{Receiver, TargetedEvent};
use std::collections::HashMap;
use std::str::FromStr;
use tracing::info;

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

    let region_folder_path = FILES
        .world_region_path(&NamespacedKey::minecraft("overworld").unwrap())
        .unwrap();

    let region = Region::new(region_folder_path.join("r.0.0.mca"));
    let view_distance: i32 = 8;
    let registry = REGISTRY.get().unwrap();
    let biome_registry_namespaced_key = NamespacedKey::minecraft("worldgen/biome").unwrap();

    for x in 0..17 {
        for z in 0..17 {
            let blocks_ids_vec = region
                .read_chunk(x, z, |b| {
                    registry
                        .get_numeric_id(&biome_registry_namespaced_key, &NamespacedKey::from_str(b).unwrap())
                        .unwrap() as u64
                })
                .expect("Anvil format error");

            let mut bytes = BytesMut::new();

            for (blocks_ids, biomes) in blocks_ids_vec {
                let section = ChunkSection::new(blocks_ids.clone(), biomes.clone());
                section.serialize_to_mc_format(&mut bytes).unwrap();
            }

            let heightmap = nbt!("", {
                "MOTION_BLOCKING": [L;],
                "WORLD_SURFACE": [L;],
            });

            // Temporary implementation of light
            let sky_light_mask = BitSet::new(vec![true; CHUNK_SECTIONS_PER_WORLD_HEIGHT + 2]);
            let chunk_section_sky_light_array = Bytes::from(vec![u8::MAX; AMOUNT_OF_BLOCKS_IN_CHUNK_SECTION / 2]);
            let chunk_sky_light_array = vec![chunk_section_sky_light_array; CHUNK_SECTIONS_PER_WORLD_HEIGHT + 2];

            connection.send_packet(ChunkDataAndUpdateLightS2CPacket {
                chunk_x: x as i32,
                chunk_z: z as i32,
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
        position: Position::new(100.0, 70.0, 100.0),
        yaw: 0.0,
        pitch: 0.0,
        flags: SynchronizePositionFlags::new(),
        teleport_id: 2137,
    });

    info!("{name} joined!");
}
