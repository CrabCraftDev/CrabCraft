use crate::anvil::chunk_format;
use crate::anvil::chunk_format::ANVIL_DATA_VERSION;
use crate::anvil::region::anvil_compression::AnvilCompression;
use crate::anvil::region::header::RegionHeader;
use crate::anvil::sectors::{SECTOR_SIZE, SectorAllocator};
use crate::chunks::chunk::Chunk;
use crate::error::AnvilError;
use crate::error::AnvilError::ChunkDoesNotExist;
use crate::utils::{MEGABYTE, read_u32};
use crate::world::World;
use crab_nbt::serde::de::from_bytes;
use crab_nbt::serde::ser::to_bytes;
use crab_registry::REGISTRY;
use crab_registry::minecraft::Biome;
use crab_text::utils::namespaced_key::NamespacedKey;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

/// The Region file format is a binary format used to store Minecraft chunks.
///
/// It consists of:
/// - A header containing metadata for each chunk in the file
/// - Multiple 4096-byte sectors, each capable of storing part of a chunk
///
/// Chunks are compressed using a specified compression method within their sectors.
///
/// For details, see the [Minecraft Wiki](https://minecraft.wiki/w/Region_file_format).
#[derive(Debug)]
pub struct RegionFile {
    file_handle: File,
    header: RegionHeader,
    allocator: SectorAllocator,
}

impl RegionFile {
    pub fn read_chunk(&mut self, x: u8, z: u8, get_biome: impl Fn(&str) -> u64) -> Result<Chunk, AnvilError> {
        let sector = self.header.sector(x, z);
        if !sector.exists() {
            return Err(ChunkDoesNotExist { x, z });
        }

        self.file_handle.seek(SeekFrom::Start(sector.binary_offset()))?;

        let compressed_len = read_u32(&mut self.file_handle)? as usize;
        if compressed_len > MEGABYTE || compressed_len == 0 {
            return Err(AnvilError::InvalidChunkSize { size: compressed_len });
        }

        let mut compressed_content = vec![0u8; compressed_len];
        self.file_handle.read_exact(&mut compressed_content)?;

        let mut decompressed = AnvilCompression::decompress(compressed_content)?;

        let anvil_chunk = from_bytes::<chunk_format::AnvilChunk>(&mut decompressed)?;
        // println!("{:?}", anvil_chunk);
        let chunk = anvil_chunk.to_chunk(get_biome)?;
        Ok(chunk)
    }

    pub fn write_chunk(&mut self, x: i32, z: i32, chunk: &Chunk) -> Result<(), AnvilError> {
        let (local_chunk_x, local_chunk_z) = World::chunk_coordinates_inside_region(x, z);
        let sector = self.header.sector(local_chunk_x, local_chunk_z);
        if sector.exists() {
            self.allocator.free(sector);
        }

        let root_registry = REGISTRY.load();
        let registry = root_registry.get_registry(Biome::REGISTRY).unwrap();

        // FIXME: The values below are placeholders and should be replaced with actual values
        let anvil_chunk = chunk_format::AnvilChunk::from_chunk(
            chunk.clone(),
            |biome_id| registry.id_to_key(biome_id as usize).unwrap().to_string(),
            x,
            z,
            -4,
            ANVIL_DATA_VERSION,
            NamespacedKey::minecraft("full").unwrap(),
            0,
        )?;
        let content = to_bytes(&anvil_chunk, "".to_string())?;
        let compressed_content = AnvilCompression::compress(&content, AnvilCompression::Zlib)?;

        let compressed_len = compressed_content.len();
        if compressed_len > MEGABYTE || compressed_len == 0 {
            return Err(AnvilError::InvalidChunkSize { size: compressed_len });
        }

        let total_len = compressed_len + size_of::<u32>();
        let sectors_amount = total_len.div_ceil(SECTOR_SIZE);

        let new_sector = self.allocator.allocate(sectors_amount);

        self.file_handle.seek(SeekFrom::Start(new_sector.binary_offset()))?;
        self.file_handle.write_all(&(compressed_len as u32).to_be_bytes())?;
        self.file_handle.write_all(&compressed_content)?;

        let padding_length = SECTOR_SIZE - total_len % SECTOR_SIZE;
        self.file_handle.write_all(&vec![0u8; padding_length])?;

        self.header.set_sector(local_chunk_x, local_chunk_z, new_sector);
        self.save_header()?;

        Ok(())
    }

    fn save_header(&mut self) -> Result<(), AnvilError> {
        self.file_handle.seek(SeekFrom::Start(0))?;
        self.header.write(&mut self.file_handle)?;
        Ok(())
    }

    pub fn load(mut file_handle: File) -> Result<Self, AnvilError> {
        let header = RegionHeader::load(&mut file_handle)?;

        let file_size = file_handle.metadata()?.len() as usize;
        let amount_of_sectors = file_size.div_ceil(SECTOR_SIZE);

        let mut allocator = SectorAllocator::new(amount_of_sectors);
        for chunk_location in &header.locations {
            if !chunk_location.exists() {
                continue;
            }

            let offset = chunk_location.offset();
            let count = chunk_location.size();
            for sector in offset..(offset + count as u32) {
                allocator.force_set(sector as usize);
            }
        }

        // The first 2 sectors are reserved for the header
        allocator.force_set(0);
        allocator.force_set(1);

        Ok(Self {
            file_handle,
            header,
            allocator,
        })
    }
}
