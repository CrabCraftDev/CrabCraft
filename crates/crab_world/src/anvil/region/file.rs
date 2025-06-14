use crate::anvil::chunk_format;
use crate::anvil::region::compression::Compression;
use crate::anvil::region::header::RegionHeader;
use crate::anvil::sectors::{SECTOR_SIZE, SectorAllocator};
use crate::chunks::chunk::Chunk;
use crate::error::AnvilError;
use crate::error::AnvilError::ChunkDoesNotExist;
use crate::utils::read_u32;
use bytes::{Buf, Bytes};
use crab_nbt::serde::de::from_bytes;
use crab_registry::RegistryManager;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

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

        self.file_handle
            .seek(SeekFrom::Start(u64::from(sector.offset()) * SECTOR_SIZE as u64))?;

        let compressed_len = read_u32(&mut self.file_handle)? as usize;
        // todo: proper size
        // if compressed_len > 1_048_576 || compressed_len == 0 {
        //     return Err(AnvilError::ChunkDoesNotExist { x, z });
        // }

        let mut compressed_content = vec![0u8; compressed_len];
        self.file_handle.read_exact(&mut compressed_content)?;
        let mut content_compressed = Bytes::from(compressed_content);

        let compression_scheme = content_compressed.get_u8();

        let compression = Compression::from_repr(compression_scheme)
            .ok_or(AnvilError::InvalidCompressionScheme(compression_scheme))?;
        let mut decompressed = compression.decompress(content_compressed)?;

        let anvil_chunk = from_bytes::<chunk_format::AnvilChunk>(&mut decompressed)?;
        let chunk = anvil_chunk.to_chunk(get_biome, registry)?;
        Ok(chunk)
    }

    pub fn load(mut file_handle: File) -> Result<Self, AnvilError> {
        let header = RegionHeader::load(&mut file_handle)?;

        let file_size = file_handle.metadata()?.len() as usize;

        let mut allocator = SectorAllocator::new(file_size);
        for chunk_location in &header.locations {
            if !chunk_location.exists() {
                continue;
            }

            let offset = chunk_location.offset();
            let count = chunk_location.count();
            for sector in offset..offset + count as u32 {
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
