use crab_world::anvil::region::file::RegionFile;
use std::fs::{self, File};
use std::path::PathBuf;

#[test]
fn test_read_write_read_all_chunks() {
    let src = PathBuf::from("tests/data/r.0.0.mca");
    let dst = std::env::temp_dir().join("r.0.0.test.mca");
    fs::copy(&src, &dst).expect("failed to copy region file");

    let file = File::options().read(true).write(true).open(&dst).unwrap();
    let mut region = RegionFile::load(file).unwrap();

    let mut original_chunks = Vec::new();
    for x in 0..8 {
        for z in 0..8 {
            let chunk = region.read_chunk(x, z).unwrap();
            original_chunks.push(((x, z), chunk));
        }
    }

    for ((x, z), chunk) in &original_chunks {
        region
            .write_chunk(*x as i32, *z as i32, chunk)
            .expect("Failed to write chunk");
    }

    let file2 = File::open(&dst).unwrap();
    let mut region2 = RegionFile::load(file2).unwrap();

    for ((x, z), original_chunk) in &original_chunks {
        let reread_chunk_opt = region2.read_chunk(*x, *z).unwrap();

        assert_eq!(
            format!("{:?}", original_chunk),
            format!("{:?}", reread_chunk_opt),
            "Chunk data mismatch at ({}, {})",
            x,
            z
        );
    }

    let _ = fs::remove_file(&dst);
}
