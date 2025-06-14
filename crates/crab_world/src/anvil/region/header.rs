use crate::anvil::sectors::{SECTOR_SIZE, Sector};
use crate::error::AnvilError;
use crate::utils::read_u32;
use std::fs::File;

/// Header of a region file in the Anvil format.
/// It is used mostly to locate chunks within the file
pub struct RegionHeader {
    /// Locations of chunks in the file, relative to the start.
    pub locations: Vec<Sector>,
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
