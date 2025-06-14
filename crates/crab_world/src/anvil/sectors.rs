use bitfield_struct::bitfield;
use bitvec::vec::BitVec;

pub const SECTOR_SIZE: usize = 4096;

#[bitfield(u32)]
pub struct Sector {
    pub count: u8,
    #[bits(24)]
    pub offset: u32,
}

impl Sector {
    /// Returns `true` if the sector has non-zero count and offset,
    /// indicating the chunk exists in a Region file.
    pub fn exists(self) -> bool {
        self.count() != 0 && self.offset() != 0
    }
}

/// Manages allocation of 4096-byte sectors in a region file.
/// Each chunk can occupy multiple contiguous sectors.
pub struct SectorAllocator {
    used_sectors: BitVec,
}

impl SectorAllocator {
    pub fn new(sectors_count: usize) -> Self {
        Self {
            used_sectors: BitVec::repeat(false, sectors_count),
        }
    }

    pub fn force_set(&mut self, sector: usize) {
        self.used_sectors.set(sector, true);
    }
}
