use crate::components::name::Name;
use crate::components::uuid::UuidComponent;
use crate::events::player::player_join_world::PlayerJoinWorldEvent;
use crate::network::connection::NetworkConnection;
use crate::world::OVERWORLD_IDENTIFIER;
use crate::world::worlds_storage::WORLDS_STORAGE;
use bytes::{Bytes, BytesMut};
use crab_nbt::nbt;
use crab_protocol::packets::play::{
    ChunkBatchFinishedS2CPacket, ChunkBatchStartS2CPacket, ChunkDataAndUpdateLightS2CPacket, GameEventS2CPacket,
    GameJoinS2CPacket, LightData, PlayerInfoAction, PlayerInfoUpdateS2CPacket, PlayerPositionS2CPacket,
    SetCenterChunkS2CPacket, SetDefaultSpawnPositionS2CPacket, SpawnEntityS2CPacket, SystemChatMessageS2CPacket,
};
use crab_protocol::types::angle::Angle;
use crab_protocol::types::bit_set::{BitSet, FixedBitSet};
use crab_protocol::types::block_position::BlockPosition;
use crab_protocol::types::game_mode::GameMode;
use crab_protocol::types::player_spawn_information::PlayerSpawnInformation;
use crab_protocol::types::position::{Position, Rotation, Vec3Double};
use crab_text::text::Text;
use crab_world::chunks::chunk::CHUNK_SECTIONS_PER_WORLD_HEIGHT;
use crab_world::chunks::section::AMOUNT_OF_BLOCKS_IN_CHUNK_SECTION;
use evenio::entity::EntityId;
use evenio::event::Receiver;
use evenio::fetch::Fetcher;
use std::collections::HashMap;
use tracing::info;

#[allow(clippy::type_complexity)]
pub fn on_world_join(
    receiver: Receiver<
        PlayerJoinWorldEvent,
        (
            &NetworkConnection,
            &Name,
            &Position,
            &Rotation,
            &UuidComponent,
            EntityId,
        ),
    >,
    fetcher: Fetcher<(
        &NetworkConnection,
        &UuidComponent,
        &Name,
        EntityId,
        &Position,
        &Rotation,
    )>,
) {
    let (connection, name, position, rotation, uuid, id) = receiver.query;

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
    let mut to_everyone = HashMap::new();
    to_everyone.insert(
        uuid.0,
        vec![
            PlayerInfoAction::AddPlayer {
                name: name.0.clone(),
                properties: vec![],
            },
            PlayerInfoAction::UpdateListed(true),
        ],
    );
    let mut sync_other = HashMap::new();

    let join_message = Text::from(format!("{name} joined!"));
    for (conn, puuid, pname, _pid, _ppos, _prot) in fetcher.iter() {
        sync_other.insert(
            puuid.0,
            vec![
                PlayerInfoAction::AddPlayer {
                    name: pname.0.clone(),
                    properties: vec![],
                },
                PlayerInfoAction::UpdateListed(true),
            ],
        );
        conn.send_packet(PlayerInfoUpdateS2CPacket {
            actions: FixedBitSet::<8>(BitSet::new(vec![true, false, false, true, false, false, false, false])),
            player_actions: to_everyone.clone(),
        });
        conn.send_packet(SystemChatMessageS2CPacket {
            content: join_message.clone(),
            is_actionbar: false,
        });
        if uuid != puuid {
            conn.send_packet(SpawnEntityS2CPacket {
                entity_id: id.index().0 as i32,
                entity_uuid: uuid.0,
                entity_type: 147, // its player entity trust me
                position: position.clone(),
                pitch: Angle(rotation.pitch as u8),
                yaw: Angle(rotation.yaw as u8),
                head_yaw: Angle(0),
                data: 0,
                velocity_x: 0,
                velocity_y: 0,
                velocity_z: 0,
            });
        }
    }
    connection.send_packet(PlayerInfoUpdateS2CPacket {
        actions: FixedBitSet::<8>(BitSet::new(vec![true, false, false, true, false, false, false, false])),
        player_actions: sync_other.clone(),
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

            let chunk = dimension.get_chunk_or_load(x, z).unwrap();
            for section in &chunk.sections {
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
                chunk_x: x,
                chunk_z: z,
                heightmaps: heightmap,
                data: bytes.freeze(),
                block_entities: chunk.block_entities.clone(),
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

    for (_conn, puuid, _pname, pid, ppos, prot) in fetcher {
        if uuid == puuid {
            continue;
        }
        connection.send_packet(SpawnEntityS2CPacket {
            entity_id: pid.index().0 as i32,
            entity_uuid: puuid.0,
            entity_type: 147, // its player entity trust me
            position: ppos.clone(),
            pitch: Angle(prot.pitch as u8),
            yaw: Angle(prot.yaw as u8),
            head_yaw: Angle(0),
            data: 0,
            velocity_x: 0,
            velocity_y: 0,
            velocity_z: 0,
        });
    }

    info!("{name} joined!");
}
