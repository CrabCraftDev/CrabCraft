use crate::network::connection::NetworkConnection;
use crate::world::worlds_storage::WORLDS_STORAGE;
use crate::world::{OVERWORLD_IDENTIFIER, World};
use bytes::{Bytes, BytesMut};
use crab_nbt::nbt;
use crab_protocol::packets::play::{ChunkBatchFinishedS2CPacket, ChunkDataAndUpdateLightS2CPacket, LightData};
use crab_protocol::types::bit_set::BitSet;
use crab_protocol::types::position::Position;
use crab_text::utils::namespaced_key::NamespacedKey;
use crab_world::chunks::chunk::CHUNK_SECTIONS_PER_WORLD_HEIGHT;
use crab_world::chunks::section::AMOUNT_OF_BLOCKS_IN_CHUNK_SECTION;
use dashmap::mapref::one::RefMut;
use std::collections::HashMap;
use crossbeam::queue::SegQueue;
use tracing::warn;

pub const VIEW_DISTANCE: i32 = 12;

pub struct ChunkLoadManager {
    pub player_chunk_tickets: HashMap<(i32, i32), usize>
}

impl ChunkLoadManager {
    pub fn tick(&mut self) {
        let mut dimension = WORLDS_STORAGE.worlds.get_mut(&OVERWORLD_IDENTIFIER).unwrap();
        for ((x, z), tickets) in &self.player_chunk_tickets {
            if tickets > &0 {
                if dimension.get_chunk(*x, *z).is_none() {
                    dimension.load_chunk(*x, *z).expect("kurwa zjebalo sie");
                }
            } else {
                dimension.unload_chunk(*x, *z);
            }
        }
    }
    pub fn on_player_move(&mut self, old_pos: &Position, new_pos: &Position) {
        self.remove_player_tickets(old_pos);
        self.add_player_tickets(new_pos);
    }
    pub fn add_player_tickets(&mut self, position: &Position) {
        let center_chunk_x = position.0.x as i32 % 16;
        let center_chunk_z = position.0.x as i32 % 16;

        for x in 0..VIEW_DISTANCE {
            for z in 0..VIEW_DISTANCE {
                self.add_player_chunk_ticket(center_chunk_x + x, center_chunk_z + z);
                self.add_player_chunk_ticket(center_chunk_x + x, center_chunk_z - z);
                self.add_player_chunk_ticket(center_chunk_x - x, center_chunk_z - z);
                self.add_player_chunk_ticket(center_chunk_x - x, center_chunk_z + z);
            }
        }
    }

    pub fn remove_player_tickets(&mut self, position: &Position) {
        let center_chunk_x = position.0.x as i32 % 16;
        let center_chunk_z = position.0.x as i32 % 16;

        for x in 0..VIEW_DISTANCE {
            for z in 0..VIEW_DISTANCE {
                self.remove_player_chunk_ticket(center_chunk_x + x, center_chunk_z + z);
                self.remove_player_chunk_ticket(center_chunk_x + x, center_chunk_z - z);
                self.remove_player_chunk_ticket(center_chunk_x - x, center_chunk_z - z);
                self.remove_player_chunk_ticket(center_chunk_x - x, center_chunk_z + z);
            }
        }
    }
    fn add_player_chunk_ticket(&mut self, x: i32, z: i32) {
        self.player_chunk_tickets
            .entry(World::get_key(x, z)).and_modify(|c| *c += 1).or_insert(1);
    }

    fn remove_player_chunk_ticket(&mut self, x: i32, z: i32) {
        let count = self.player_chunk_tickets
            .entry(World::get_key(x, z))
            .and_modify(|c| *c -= 1);
        if *count.or_default() == 0 {
            self.player_chunk_tickets.remove(&World::get_key(x, z));
        }
    }
}

pub fn send_chunks_on_join(connection: &NetworkConnection, player_pos: &Position) {
    let mut dimension = WORLDS_STORAGE.worlds.get_mut(&OVERWORLD_IDENTIFIER).unwrap();

    let center_chunk_x = player_pos.0.x as i32 % 16;
    let center_chunk_z = player_pos.0.z as i32 % 16;
    let mut number_of_sent_chunks = 0;
    for x in 0..VIEW_DISTANCE {
        for z in 0..VIEW_DISTANCE {
            send_chunk(connection, center_chunk_x + x, center_chunk_z + z, &mut dimension);
            send_chunk(connection, center_chunk_x + x, center_chunk_z - z, &mut dimension);
            send_chunk(connection, center_chunk_x - x, center_chunk_z - z, &mut dimension);
            send_chunk(connection, center_chunk_x - x, center_chunk_z + z, &mut dimension);
            number_of_sent_chunks += 1;
        }
    }

    connection.send_packet(ChunkBatchFinishedS2CPacket {
        number_of_chunks: number_of_sent_chunks,
    });
}
pub fn send_chunks_on_movement(connection: &NetworkConnection, old_pos: &mut Position, new_pos: &Position) {
    // updateChunkScheduling
    // connection.send_packet(SetCenterChunkS2CPacket { x: 0, z: 0 });
}

fn remove_ticket(p0: i32, p1: i32, p2: &mut RefMut<NamespacedKey, World>) {
    todo!()
}

pub fn send_chunk(connection: &NetworkConnection, x: i32, z: i32, dimension: &mut RefMut<NamespacedKey, World>) {
    let mut bytes = BytesMut::new();
    let chunk_coords = World::get_key(x, z);

    dimension
        .chunk_tickets
        .entry(chunk_coords)
        .and_modify(|counter| *counter += 1)
        .or_insert(1);

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
