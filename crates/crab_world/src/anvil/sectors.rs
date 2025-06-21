use bitfield_struct::bitfield;
use bitvec::vec::BitVec;

pub const SECTOR_SIZE: usize = 4096;

#[bitfield(u32)]
pub struct Sector {
    /// Number of 4KB sectors this chunk occupies.
    pub size: u8,
    /// Starting sector's offset of this chunk within the region file.
    #[bits(24)]
    pub offset: u32,
}

impl Sector {
    /// Returns `true` if the sector has non-zero count and offset,
    /// indicating the chunk exists in a Region file.
    pub fn exists(self) -> bool {
        self.size() != 0 && self.offset() != 0
    }

    pub fn binary_offset(self) -> u64 {
        u64::from(self.offset()) * SECTOR_SIZE as u64
    }
}

/// Manages allocation of 4096-byte sectors in a region file.
/// Each chunk can occupy multiple contiguous sectors.
#[derive(Debug)]
pub struct SectorAllocator {
    used_sectors: BitVec,
}

impl SectorAllocator {
    pub fn new(sectors_count: usize) -> Self {
        Self {
            used_sectors: BitVec::repeat(false, sectors_count),
        }
    }

    pub fn force_set(&mut self, index: usize) {
        self.used_sectors.set(index, true);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sector_exists_returns_true_when_size_and_offset_nonzero() {
        let sector = Sector::new().with_size(2).with_offset(10);
        assert!(sector.exists());
    }

    #[test]
    fn sector_exists_returns_false_when_size_is_zero() {
        let sector = Sector::new().with_size(0).with_offset(10);
        assert!(!sector.exists());
    }

    #[test]
    fn sector_exists_returns_false_when_offset_is_zero() {
        let sector = Sector::new().with_size(2).with_offset(0);
        assert!(!sector.exists());
    }

    #[test]
    fn sector_binary_offset_calculates_correct_offset() {
        let sector = Sector::new().with_size(1).with_offset(3);
        assert_eq!(sector.binary_offset(), 3 * SECTOR_SIZE as u64);
    }

    #[test]
    fn sector_allocator_new_initializes_all_sectors_unused() {
        let allocator = SectorAllocator::new(5);
        assert_eq!(allocator.used_sectors.count_ones(), 0);
    }

    #[test]
    fn sector_allocator_force_set_marks_sector_as_used() {
        let mut allocator = SectorAllocator::new(3);
        allocator.force_set(1);
        assert!(allocator.used_sectors[1]);
        assert_eq!(allocator.used_sectors.count_ones(), 1);
    }

    #[test]
    #[should_panic]
    fn sector_allocator_force_set_panics_on_out_of_bounds() {
        let mut allocator = SectorAllocator::new(2);
        allocator.force_set(2);
    }
}
