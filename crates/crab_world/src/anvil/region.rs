use crate::error::AnvilError;
use bitfield_struct::bitfield;
use bytes::{Buf, Bytes};
use crab_nbt::Nbt;
use crab_world::blocks::Block;
use crab_world::chunks::bit_storage::BitStorage;
use crab_world::chunks::palette::palette_content_type::PaletteContentType;
use crab_world::chunks::palette::paletted_storage::{Palette, PalettedStorage};
use flate2::read::{GzDecoder, ZlibDecoder};
use std::collections::HashMap;
use std::io::Read;
use std::str::FromStr;
use strum::FromRepr;

use crate::chunks::chunk::Chunk;
use crate::chunks::section::{AMOUNT_OF_BIOMES_IN_CHUNK_SECTION, AMOUNT_OF_BLOCKS_IN_CHUNK_SECTION};
use crate::error::AnvilError::{ChunkNotGenerated, NbtTagNotFound};
use crab_protocol::packets::play::BlockEntity;
use crab_registry::RegistryManager;
use crab_text::utils::namespaced_key::NamespacedKey;
use crab_world::chunks::section::ChunkSection;
use crab_world::error::AnvilError::BlockEntityDoesNotExist;
use lz4::Decoder;

#[bitfield(u32)]
struct Location {
    count: u8,
    #[bits(24)]
    offset: u32,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, FromRepr)]
#[repr(u8)]
pub enum Compression {
    Gzip = 1,
    #[default]
    Zlib = 2,
    Uncompressed = 3,
    LZ4 = 4,
    Custom = 127,
}

pub struct Region {
    locations: Vec<Location>,
    bytes: Bytes,
}

const SECTOR_SIZE: usize = 4096;

impl Region {
    pub fn read_chunk(
        &self,
        x: u8,
        z: u8,
        get_biome: impl Fn(&str) -> u64,
        registry: &RegistryManager,
    ) -> Result<Chunk, AnvilError> {
        let index: usize = x as usize + (z as usize) * 32;
        let location = self.locations[index];
        if location.offset() == 0 && location.count() == 0 {
            return Err(ChunkNotGenerated { x, z });
        }

        // The First 2 sectors are locations and timestamps tables, but we don't store them
        let start = (location.offset() - 2) as usize * SECTOR_SIZE;

        let mut bytes = self.bytes.slice(start..(start + 4));
        let compressed_len = bytes.get_u32();
        let mut content_compressed = self
            .bytes
            .slice((start + 4)..(start + 4usize + compressed_len as usize));

        let compression_scheme = content_compressed.get_u8();

        let mut decompressed_data = Vec::new();
        let mut content = match Compression::from_repr(compression_scheme) {
            Some(Compression::Gzip) => {
                let mut z = GzDecoder::new(&content_compressed[..]);
                z.read_to_end(&mut decompressed_data)?;
                Bytes::from(decompressed_data)
            }
            Some(Compression::Zlib) => {
                let mut z = ZlibDecoder::new(&content_compressed[..]);
                z.read_to_end(&mut decompressed_data)?;
                Bytes::from(decompressed_data)
            }
            Some(Compression::LZ4) => {
                let mut z = Decoder::new(&content_compressed[..])?;
                z.read_to_end(&mut decompressed_data)?;
                Bytes::from(decompressed_data)
            }
            Some(Compression::Uncompressed) => content_compressed,
            _ => return Err(AnvilError::InvalidCompressionScheme(compression_scheme)),
        };

        let nbt = Nbt::read(&mut content).unwrap();

        let mut chunk = Chunk::without_sections();

        chunk.block_entities = Self::load_block_entities(&nbt, registry)?;

        let sections = nbt.get_list("sections").unwrap();
        for section in sections.iter() {
            let section = section.extract_compound().unwrap();

            let block_states = section.get_compound("block_states").unwrap();
            let mut palette = Vec::new();
            for entry in block_states.get_list("palette").unwrap() {
                let entry = entry.extract_compound().unwrap();
                let mut properties = HashMap::new();
                if let Some(props) = entry.get_compound("Properties") {
                    for (property, value) in &props.child_tags {
                        properties.insert(property.as_str(), value.extract_string().unwrap().clone());
                    }
                }
                let name = entry.get_string("Name").unwrap();
                let block_identifier = &NamespacedKey::from_str(name).unwrap();

                let block_id = Block::from_properties(block_identifier, &properties).unwrap();
                palette.push(block_id.0 as u32);
            }
            let block_states_values = block_states.get_long_array("data").unwrap_or(&vec![]).clone();
            let storage = BitStorage::new(AMOUNT_OF_BLOCKS_IN_CHUNK_SECTION, block_states_values);
            let blocks_storage = PalettedStorage::new(
                PaletteContentType::Blocks,
                Palette::new(palette, storage, &PaletteContentType::Blocks),
            );

            let biomes = section.get_compound("biomes").unwrap();
            let mut palette = Vec::new();
            for entry in biomes.get_list("palette").unwrap() {
                palette.push(get_biome(entry.extract_string().unwrap()) as u32);
            }
            let biomes_states_values = biomes.get_long_array("data").unwrap_or(&vec![]).clone();
            let storage = BitStorage::new(AMOUNT_OF_BIOMES_IN_CHUNK_SECTION as usize, biomes_states_values);
            let biomes_storage = PalettedStorage::new(
                PaletteContentType::Biomes,
                Palette::new(palette, storage, &PaletteContentType::Biomes),
            );

            chunk.sections.push(ChunkSection::new(blocks_storage, biomes_storage));
        }

        Ok(chunk)
    }

    pub fn load_block_entities(nbt: &Nbt, registry: &RegistryManager) -> Result<Vec<BlockEntity>, AnvilError> {
        let vec_binding = Vec::new();
        let block_entities = nbt.get_list("block_entities").unwrap_or(&vec_binding);
        let mut block_entities_vec = Vec::new();
        for block_entity in block_entities {
            let block_entity = block_entity.extract_compound().ok_or(NbtTagNotFound)?;
            let x = block_entity.get_int("x").ok_or(NbtTagNotFound)?;
            let y = block_entity.get_int("y").ok_or(NbtTagNotFound)? as i16;
            let z = block_entity.get_int("z").ok_or(NbtTagNotFound)?;

            let id = NamespacedKey::from_str(block_entity.get_string("id").ok_or(NbtTagNotFound)?)?;
            let numeric_id = registry.block_entities.get(&id).ok_or(BlockEntityDoesNotExist(id))?;

            let block_entity = BlockEntity {
                relative_x: (x % 16) as i8,
                relative_z: (z % 16) as i8,
                y,
                block_entity_type: *numeric_id as i32,
                // todo: Some fields are not serialized, because it depends on the block entity type,
                // for now we send everything to the client
                data: Nbt::from(block_entity.clone()),
            };

            block_entities_vec.push(block_entity);
        }
        Ok(block_entities_vec)
    }
}

impl From<Bytes> for Region {
    fn from(mut bytes: Bytes) -> Self {
        let mut locations = Vec::new();
        for _ in 0..1024 {
            let location = Location(bytes.get_u32());
            locations.push(location);
        }
        let _timestamps = bytes.copy_to_bytes(SECTOR_SIZE);

        Self { locations, bytes }
    }
}
