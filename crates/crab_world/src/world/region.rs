use crate::error::WorldError;
use crate::world::World;
use crab_world::anvil::region::file::RegionFile;
use dashmap::mapref::one::RefMut;
use std::fs::OpenOptions;

impl World {
    pub fn get_or_load_region_file(&self, x: i32, z: i32) -> Result<RefMut<'_, (i32, i32), RegionFile>, WorldError> {
        let region_file_name = Self::region_file_name(x, z);
        let region_coordinates = Self::region_coordinates(x, z);

        if !self.regions.contains_key(&region_coordinates) {
            let file = OpenOptions::new()
                .read(true)
                .write(true)
                .open(self.path.join(&region_file_name))
                .map_err(|_| WorldError::MissingWorldFile(region_file_name.clone()))?;

            let region_file = RegionFile::load(file)?;
            self.regions.insert(region_coordinates, region_file);
        }

        Ok(self.regions.get_mut(&region_coordinates).unwrap())
    }

    fn region_coordinates(x: i32, z: i32) -> (i32, i32) {
        // Floor of dividing the chunk coordinates by 32 (REGION_SIZE_IN_CHUNKS)
        (x >> 5, z >> 5)
    }

    fn region_file_name(chunk_x: i32, chunk_z: i32) -> String {
        let (region_x, region_z) = Self::region_coordinates(chunk_x, chunk_z);
        format!("r.{}.{}.mca", region_x, region_z)
    }
}

#[cfg(test)]
mod tests {
    use crate::world::World;

    #[test]
    fn test_get_region_name() {
        let region = World::region_file_name(1500, -600);

        assert_eq!(region, "r.46.-19.mca");
    }
}
