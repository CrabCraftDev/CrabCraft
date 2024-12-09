use crate::events::global::world::{ChunksLoadedEvent, WorldTickEvent};
use crate::events::player::movement::PlayerMoveEvent;
use crate::mc_server::MAX_TICK_DURATION;
use crate::network::connection::NetworkConnection;
use crate::world::chunk_loader::{VIEW_DISTANCE, WORLD_LOADER};
use bytes::{Bytes, BytesMut};
use crab_nbt::nbt;
use crab_protocol::packets::play::{
    ChunkBatchFinishedS2CPacket, ChunkBatchStartS2CPacket, ChunkDataAndUpdateLightS2CPacket, LightData,
};
use crab_protocol::types::bit_set::BitSet;
use crab_protocol::types::namespaced_key::NamespacedKey;
use crab_protocol::types::position::Position;
use crab_world::chunks::chunk::CHUNK_SECTIONS_PER_WORLD_HEIGHT;
use crab_world::chunks::section::AMOUNT_OF_BLOCKS_IN_CHUNK_SECTION;
use evenio::entity::EntityId;
use evenio::event::{Receiver, Sender};
use evenio::fetch::Fetcher;

pub fn tick(receiver: Receiver<WorldTickEvent>, sender: Sender<ChunksLoadedEvent>) {
    let WorldTickEvent { tick_start } = receiver.event;

    let dimension = NamespacedKey::minecraft("overworld").unwrap();
    
    let chunk_loader = &mut WORLD_LOADER.chunk_loaders.get_mut(&dimension).unwrap();

    let tickets_map = chunk_loader.value().tickets.clone();
    let mut priority = 0;
    let mut chunks = vec![];

    while tick_start.elapsed().as_millis() + 10 < MAX_TICK_DURATION.as_millis() {
        let mut tickets = tickets_map.get_mut(&priority).unwrap();
        if tickets.len() == 0 {
            priority += 1;
            if priority == 255 {
                break;
            }
            continue;
        }
        let mut index = 0;
        let mut remove_indexes = vec![];
        for ticket in tickets.iter_mut() {
            let Some((x, z)) = ticket.next() else {
                remove_indexes.push(index);
                continue;
            };
            chunk_loader.load_chunk_now(x, z);
            chunks.push((x, z));
            index += 1;
        }
        drop(tickets); // this drop is required to not deadlock the server
        for i in remove_indexes {
            let ticket = tickets_map.get_mut(&priority).unwrap().swap_remove(i);
            tickets_map.get_mut(&ticket.priority()).unwrap().push(ticket);
        }
    }
    if !chunks.is_empty() {
        sender.send(ChunksLoadedEvent { dimension, chunks })
    }
}

pub fn prepare_chunk_send(receiver: Receiver<ChunksLoadedEvent>, fetcher: Fetcher<(&NetworkConnection, &Position)>) {
    let ChunksLoadedEvent { dimension, chunks } = receiver.event;

    let players = fetcher.iter();
    let chunk_loader = &mut WORLD_LOADER.chunk_loaders.get_mut(dimension).unwrap();

    for (connection, position) in players {
        let mut number_of_chunks = 0;
        connection.send_packet(ChunkBatchStartS2CPacket);
        for (x, z) in chunks {
            if i32::abs(x - position.chunk_x()) > VIEW_DISTANCE as i32
                || i32::abs(z - position.chunk_z()) > VIEW_DISTANCE as i32
            {
                continue;
            }
            number_of_chunks += 1;

            let mut bytes = BytesMut::new();

            chunk_loader
                .load_chunk_now(*x, *z)
                .clone()
                .serialize_to_mc_format(&mut bytes)
                .unwrap();

            let heightmap = nbt!("", {
                "MOTION_BLOCKING": [L;],
                "WORLD_SURFACE": [L;],
            });

            // Temporary implementation of light
            let sky_light_mask = BitSet::new(vec![true; CHUNK_SECTIONS_PER_WORLD_HEIGHT + 2]);
            let chunk_section_sky_light_array = Bytes::from(vec![u8::MAX; AMOUNT_OF_BLOCKS_IN_CHUNK_SECTION / 2]);
            let chunk_sky_light_array = vec![chunk_section_sky_light_array; CHUNK_SECTIONS_PER_WORLD_HEIGHT + 2];

            connection.send_packet(ChunkDataAndUpdateLightS2CPacket {
                chunk_x: *x,
                chunk_z: *z,
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
        connection.send_packet(ChunkBatchFinishedS2CPacket { number_of_chunks });
    }
}

pub fn load_chunks_on_move(receiver: Receiver<PlayerMoveEvent, (EntityId, &Position)>) {
    let PlayerMoveEvent { .. } = receiver.event;
    let (entity, position) = receiver.query;
    
    let chunk_loader = WORLD_LOADER
        .chunk_loaders
        .get(&NamespacedKey::minecraft("overworld").unwrap())
        .unwrap();

    chunk_loader.update_ticket(entity, position.chunk_x(), position.chunk_z());
}
