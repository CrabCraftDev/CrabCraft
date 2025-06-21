use crate::anvil::sectors::{SECTOR_SIZE, Sector};
use crate::error::AnvilError;
use crate::utils::read_u32;
use std::fs::File;

/// Represents the header of a region file in the Anvil format.
/// This header mainly serves to locate chunks within the file.
///
/// For details, see the [Minecraft Wiki](https://minecraft.wiki/w/Region_file_format#Header)
#[derive(Debug)]
pub struct RegionHeader {
    /// Chunk locations with offset and size for each chunk, in sectors.
    pub locations: Vec<Sector>,
    /// Last modification timestamps for each chunk, stored as Unix epoch seconds.
    pub timestamps: Vec<u32>,
}

const ENTRIES_IN_HEADER: usize = SECTOR_SIZE / size_of::<u32>();

impl RegionHeader {
    pub fn load(mut file: &mut File) -> Result<Self, AnvilError> {
        let mut header = Self {
            timestamps: Vec::with_capacity(ENTRIES_IN_HEADER),
            locations: Vec::with_capacity(ENTRIES_IN_HEADER),
        };

        for _ in 0..ENTRIES_IN_HEADER {
            let sector = Sector::from(read_u32(&mut file)?);
            header.locations.push(sector);
        }

        for _ in 0..ENTRIES_IN_HEADER {
            header.timestamps.push(read_u32(&mut file)?);
        }

        Ok(header)
    }

    pub fn sector(&self, x: u8, z: u8) -> Sector {
        let index = Self::index(x, z);
        self.locations[index]
    }

    fn index(x: u8, z: u8) -> usize {
        x as usize + (z as usize) * 32
    }
}

#[cfg(test)]
mod tests {
    use crate::anvil::region::header::RegionHeader;

    #[test]
    fn index_calculates_correct_flat_index() {
        assert_eq!(RegionHeader::index(0, 0), 0);
        assert_eq!(RegionHeader::index(31, 0), 31);
        assert_eq!(RegionHeader::index(0, 31), 992);
        assert_eq!(RegionHeader::index(31, 31), 1023);
    }
}
