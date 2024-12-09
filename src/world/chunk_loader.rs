use crate::utils::files::FILES;
use crate::world::chunk_ticket::PlayerChunkTicket;
use crab_protocol::types::namespaced_key::NamespacedKey;
use crab_registry::REGISTRY;
use crab_world::anvil::region::Region;
use crab_world::chunks::chunk::Chunk;
use crab_world::chunks::section::ChunkSection;
use dashmap::mapref::one::Ref;
use dashmap::DashMap;
use evenio::component::Component;
use evenio::entity::EntityId;
use lazy_static::lazy_static;
use nohash_hasher::BuildNoHashHasher;
use std::str::FromStr;
use std::sync::{Arc, RwLock};
use tracing::warn;

pub const VIEW_DISTANCE: u8 = 10;

lazy_static! {
    pub static ref WORLD_LOADER: WorldLoader = WorldLoader {
        chunk_loaders: Arc::new(DashMap::new())
    };
}

pub struct WorldLoader {
    pub chunk_loaders: Arc<DashMap<NamespacedKey, ChunkLoader>>,
}

impl WorldLoader {
    pub fn init_dimension(&self, dimension: NamespacedKey) {
        self.chunk_loaders
            .insert(dimension.clone(), ChunkLoader::new(dimension));
    }
}

#[derive(Component)]
pub struct ChunkLoader {
    dimension: NamespacedKey,
    data: Arc<DashMap<i64, Chunk, BuildNoHashHasher<i64>>>,
    pub tickets: Arc<DashMap<u8, Vec<PlayerChunkTicket>, BuildNoHashHasher<u8>>>, // priority (the lower, the better)
}

impl ChunkLoader {
    pub fn new(dimension: NamespacedKey) -> Self {
        let tickets = Arc::new(DashMap::with_hasher(BuildNoHashHasher::default()));
        for i in 0..=255 {
            tickets.insert(i, Vec::new());
        }
        Self {
            dimension,
            data: Arc::new(DashMap::with_hasher(BuildNoHashHasher::default())),
            tickets,
        }
    }

    pub fn load_chunk_now(&mut self, x: i32, z: i32) -> Ref<'_, i64, Chunk> {
        let key = Self::get_key(x, z);
        if let Some(chunk) = self.data.get(&key) {
            return chunk;
        }
        let Ok(region) = Region::new(
            FILES
                .world_region_path(&self.dimension)
                .unwrap()
                .join(Self::get_region_name(x, z)),
        ) else {
            warn!("CrabCraft doesn't have world generator yet!");
            warn!(
                "Please provide world files in worlds/{}/region/{}",
                self.dimension,
                Self::get_region_name(x, z)
            );
            self.data.insert(key, Chunk::default());
            return self.data.get(&key).unwrap();
        };

        let registry = REGISTRY.get().unwrap();
        let biome_registry_namespaced_key = NamespacedKey::minecraft("worldgen/biome").unwrap();

        let Ok(blocks_ids_vec) = region.read_chunk((x & 31) as u8, (z & 31) as u8, |b| {
            registry
                .get_numeric_id(&biome_registry_namespaced_key, &NamespacedKey::from_str(b).unwrap())
                .unwrap() as u64
        }) else {
            self.data.insert(key, Chunk::default());
            return self.data.get(&key).unwrap();
        };

        let mut chunk = Chunk::default();
        for (i, (blocks_ids, biomes)) in blocks_ids_vec.into_iter().enumerate() {
            chunk.sections[i] = ChunkSection::new(blocks_ids.clone(), biomes.clone());
        }

        self.data.insert(key, chunk);
        self.data.get(&key).unwrap()
    }

    /// Why would you want to unload a chunk - the more, the merrier
    #[allow(dead_code)]
    pub fn unload_chunk(&mut self, x: i32, z: i32) {
        self.data.remove(&Self::get_key(x, z));
    }

    fn get_key(x: i32, z: i32) -> i64 {
        ((x as i64) << 32) | z as i64
    }

    pub fn get_region_name(chunk_x: i32, chunk_z: i32) -> String {
        format!("r.{}.{}.mca", chunk_x >> 5, chunk_z >> 5)
    }

    pub fn add_chunk_ticket(&mut self, entity_id: EntityId, x: i32, z: i32) {
        let ticket = PlayerChunkTicket::new(entity_id, VIEW_DISTANCE as usize, x, z);
        let mut tickets = self.tickets.get_mut(&0).unwrap();
        tickets.push(ticket);
    }

    pub fn update_ticket(&self, entity_id: EntityId, x: i32, z: i32) {
        for mut tickets in self.tickets.iter_mut() {
            for ticket in tickets.iter_mut() {
                if ticket.entity_id != entity_id {
                    continue;
                }

                ticket.chunk_x = x;
                ticket.chunk_z = z;
                ticket.reset();
            }
        }
    }
}
