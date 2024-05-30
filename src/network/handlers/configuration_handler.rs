use async_trait::async_trait;
use bytes::{Bytes, BytesMut};
use crab_nbt::nbt;
use log::info;

use crab_protocol::buf::WriteByteBuf;
use crab_protocol::packets::configuration::{
    AcknowledgeFinishConfigurationC2SPacket, ClientInformationC2SPacket, FinishConfigurationS2CPacket,
    KnownPacksC2SPacket, KnownPacksS2CPacket, PluginMessageS2CPacket,
};
use crab_protocol::packets::play::{
    ChunkBatchFinishedS2CPacket, ChunkBatchStartS2CPacket, ChunkDataAndUpdateLightS2CPacket, GameEventS2CPacket,
    GameJoinS2CPacket, LightData, SetCenterChunkS2CPacket, SetDefaultSpawnPositionS2CPacket,
    SynchronizePlayerPositionS2CPacket, SynchronizePositionFlags,
};

use crab_protocol::protocol::ProtocolState;

use crate::chunks::chunk::{Chunk, AMOUNT_OF_BLOCKS_IN_CHUNK_SECTION, CHUNK_SECTIONS_PER_WORLD_HEIGHT};
use crab_protocol::raw_packet::RawPacket;
use crab_protocol::types::bit_set::BitSet;
use crab_protocol::types::block_position::BlockPosition;
use crab_protocol::types::game_mode::GameMode;
use crab_protocol::types::identifier::Identifier;
use crab_protocol::types::known_pack::KnownPack;
use crab_protocol::types::position::Position;

use crate::network::connection::Connection;
use crate::network::packet_handler::PacketHandler;
use crate::utils::errors::Error;
use crate::SERVER_VERSION;

pub struct ConfigurationHandler;

#[async_trait]
impl PacketHandler for ConfigurationHandler {
    async fn handle_packet(connection: &mut Connection, raw_packet: RawPacket) -> Result<(), Error> {
        if let Some(_packet) = raw_packet.decode::<ClientInformationC2SPacket>() {
            // Minecraft defines all biomes, damage types, and dimension types in code.
            let registry_dimension_type_packet: &[u8] =
                include_bytes!("../../../assets/packets/registry_dimension_type.bin");
            connection.send(Bytes::from(registry_dimension_type_packet)).await;
            let registry_damage_type_packet: &[u8] = include_bytes!("../../../assets/packets/registry_damage_type.bin");
            connection.send(Bytes::from(registry_damage_type_packet)).await;
            let registry_biomes_packet: &[u8] = include_bytes!("../../../assets/packets/registry_biomes.bin");
            connection.send(Bytes::from(registry_biomes_packet)).await;

            let mut data = BytesMut::new();
            data.put_string("CrabCraft");

            connection
                .send_packet(PluginMessageS2CPacket {
                    channel: Identifier::minecraft("brand")?,
                    data: data.freeze().into(),
                })
                .await;

            connection
                .send_packet(KnownPacksS2CPacket {
                    known_packs: vec![KnownPack {
                        namespace: "minecraft".to_string(),
                        id: "core".to_string(),
                        version: SERVER_VERSION.to_string(),
                    }],
                })
                .await;
        }

        if let Some(_packet) = raw_packet.decode::<KnownPacksC2SPacket>() {
            connection.send_packet(FinishConfigurationS2CPacket).await;
        }

        if let Some(_packet) = raw_packet.decode::<AcknowledgeFinishConfigurationC2SPacket>() {
            connection.state = ProtocolState::Play;

            connection
                .send_packet(GameJoinS2CPacket {
                    entity_id: 0,
                    is_hardcore: false,
                    dimension_names: vec![Identifier::minecraft("overworld")?],
                    max_players: 100,
                    view_distance: 16,
                    simulation_distance: 20,
                    reduced_debug_info: false,
                    enable_respawn_screen: true,
                    do_limited_crafting: false,
                    dimension_type: 0,
                    dimension_name: Identifier::minecraft("overworld")?,
                    hashed_seed: 99887766,
                    game_mode: GameMode::Creative,
                    previous_game_mode: None.into(),
                    is_debug_world: false,
                    is_flat_world: true,
                    last_death_location: None,
                    portal_cooldown: 20,
                    enforces_secure_chat: false,
                })
                .await;

            // TODO: Temporary implementation of chunk sending

            connection
                .send_packet(SetDefaultSpawnPositionS2CPacket {
                    location: BlockPosition { x: 0, y: 0, z: 0 },
                    angle: 0.0,
                })
                .await;

            connection.send_packet(SetCenterChunkS2CPacket { x: 0, z: 0 }).await;

            connection
                .send_packet(GameEventS2CPacket { event: 13, value: 0.0 })
                .await;

            connection.send_packet(ChunkBatchStartS2CPacket).await;

            let mut normal_chunk = Chunk::default();
            let mut air_chunk = Chunk::default();

            let grass_section = &mut normal_chunk.sections[0];
            for i in 0..16 {
                for j in 0..16 {
                    grass_section.set(i, 4, j, 9);
                    grass_section.set(i, 3, j, 10);
                    grass_section.set(i, 2, j, 10);
                    grass_section.set(i, 1, j, 10);
                    grass_section.set(i, 0, j, 79);
                }
            }

            let view_distance: i32 = 8;
            for x in -view_distance..view_distance {
                for z in -view_distance..view_distance {
                    let mut data = BytesMut::new();

                    let chunk = if x.abs() < (view_distance - 2) && z.abs() < (view_distance - 2) {
                        &mut normal_chunk
                    } else {
                        &mut air_chunk
                    };
                    chunk.serialize_to_mc_format(&mut data)?;

                    // TODO: For now we don't have good block system, so I implemented only world surface
                    let world_surface_height_map = chunk.serialize_world_surface_height_map();
                    let heightmap = nbt!("", {
                        "MOTION_BLOCKING": [L;],
                        "WORLD_SURFACE": world_surface_height_map,
                    });

                    // Temporary implementation of light
                    let sky_light_mask = BitSet::new(vec![true; CHUNK_SECTIONS_PER_WORLD_HEIGHT + 2]);
                    let chunk_section_sky_light_array =
                        Bytes::from(vec![u8::MAX; AMOUNT_OF_BLOCKS_IN_CHUNK_SECTION / 2]);
                    let chunk_sky_light_array =
                        vec![chunk_section_sky_light_array; CHUNK_SECTIONS_PER_WORLD_HEIGHT + 2];

                    connection
                        .send_packet(ChunkDataAndUpdateLightS2CPacket {
                            chunk_x: x,
                            chunk_z: z,
                            heightmaps: heightmap,
                            data: data.freeze(),
                            block_entities: vec![],
                            light_data: LightData {
                                sky_light_mask,
                                block_light_mask: Default::default(),
                                empty_sky_light_mask: Default::default(),
                                empty_block_light_mask: Default::default(),
                                sky_light_array: chunk_sky_light_array,
                                block_light_array: Default::default(),
                            },
                        })
                        .await;
                }
            }

            connection
                .send_packet(ChunkBatchFinishedS2CPacket {
                    number_of_chunks: (view_distance * 2 + 1).pow(2),
                })
                .await;

            connection
                .send_packet(SynchronizePlayerPositionS2CPacket {
                    position: Position::new(0.0, -59.0, 0.0),
                    yaw: 0.0,
                    pitch: 0.0,
                    flags: SynchronizePositionFlags::new(),
                    teleport_id: 2137,
                })
                .await;

            info!("Player joined!");
        }

        Ok(())
    }
}
