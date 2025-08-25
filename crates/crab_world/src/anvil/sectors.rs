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
    // It is a bit counterintuitive to use a BitVec here,
    // but for our use case it is fast and simpler than a Segment Tree
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

    pub fn allocate(&mut self, size: usize) -> Sector {
        let total_sectors = self.used_sectors.len();

        let mut start = 0;
        let mut consecutive_free = 0;

        for i in 0..total_sectors {
            if !self.used_sectors[i] {
                if consecutive_free == 0 {
                    start = i;
                }
                consecutive_free += 1;

                if consecutive_free == size {
                    for j in start..start + size {
                        self.used_sectors.set(j, true);
                    }
                    return Sector::new().with_size(size as u8).with_offset(start as u32);
                }
            } else {
                consecutive_free = 0;
            }
        }

        let sector = Sector::new().with_size(size as u8).with_offset(total_sectors as u32);
        self.used_sectors.extend(std::iter::repeat_n(true, size));
        sector
    }

    pub fn free(&mut self, sector: Sector) {
        let offset = sector.offset() as usize;
        for i in offset..(offset + sector.size() as usize) {
            if i < self.used_sectors.len() {
                self.used_sectors.set(i, false);
            }
        }
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

    #[test]
    fn allocate_multiple_contiguous() {
        let mut alloc = SectorAllocator::new(16);
        let a = alloc.allocate(3);
        let b = alloc.allocate(2);
        assert_eq!(a.offset(), 0);
        assert_eq!(a.size(), 3);
        assert_eq!(b.offset(), 3);
        assert_eq!(b.size(), 2);
    }

    #[test]
    fn free_allows_reuse() {
        let mut alloc = SectorAllocator::new(8);
        let a = alloc.allocate(3);
        let _b = alloc.allocate(3);
        // free middle of first allocation (simulate freeing part)
        alloc.free(Sector::new().with_size(1).with_offset(a.offset() + 1));
        // next allocate(1) should reuse the freed slot at offset a.offset()+1
        let c = alloc.allocate(1);
        assert_eq!(c.offset(), a.offset() + 1);
        assert_eq!(c.size(), 1);
        // free entire first allocation and ensure space can be reclaimed
        alloc.free(a);
        let d = alloc.allocate(3);
        assert_eq!(d.offset(), a.offset());
    }

    #[test]
    fn force_set_blocks_allocation() {
        let mut alloc = SectorAllocator::new(4);
        alloc.force_set(0);
        // first available should be at 1
        let s = alloc.allocate(1);
        assert_eq!(s.offset(), 1);
    }
}
