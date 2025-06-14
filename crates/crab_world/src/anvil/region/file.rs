use crate::anvil::chunk_format;
use crate::anvil::region::compression::AnvilCompression;
use crate::anvil::region::header::RegionHeader;
use crate::anvil::sectors::{SECTOR_SIZE, SectorAllocator};
use crate::chunks::chunk::Chunk;
use crate::error::AnvilError;
use crate::error::AnvilError::ChunkDoesNotExist;
use crate::utils::{MEGABYTE, read_u32};
use crab_nbt::serde::de::from_bytes;
use crab_registry::RegistryManager;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

/// The Region file format is a binary format used to store Minecraft chunks.
///
/// It consists of:
/// - A header containing metadata for each chunk in the file
/// - Multiple 4096-byte sectors, each capable of storing part of a chunk
///
/// Chunks are compressed using a specified compression method within their sectors.
///
/// For details, see the [Minecraft Wiki](https://minecraft.wiki/w/Region_file_format).
pub struct RegionFile {
    file_handle: File,
    header: RegionHeader,
    // Sector allocator is used for saving chunks, which is not implemented yet.
    #[allow(dead_code)]
    allocator: SectorAllocator,
}

impl RegionFile {
    pub fn read_chunk(
        &mut self,
        x: u8,
        z: u8,
        get_biome: impl Fn(&str) -> u64,
        registry: &RegistryManager,
    ) -> Result<Chunk, AnvilError> {
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
        let chunk = anvil_chunk.to_chunk(get_biome, registry)?;
        Ok(chunk)
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
