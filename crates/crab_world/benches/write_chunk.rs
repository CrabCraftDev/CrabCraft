use crab_registry::{REGISTRY, RootRegistry};
use crab_world::anvil::region::file::RegionFile;
use crab_world::blocks::block_registry::BlockInformation;
use crab_world::blocks::init_blocks;
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use std::fs::{self, OpenOptions};
use std::path::PathBuf;
use std::sync::Arc;

pub fn write_chunk(criterion: &mut Criterion) {
    let mut registry = RootRegistry::new();
    registry.register_registry(BlockInformation::REGISTRY, init_blocks());
    REGISTRY.store(Arc::new(registry));

    let src = PathBuf::from("./tests/data/r.0.0.mca");
    let dst = std::env::temp_dir().join("r.0.0.bench.mca");
    fs::copy(&src, &dst).expect("failed to copy region file");

    let file = OpenOptions::new().read(true).write(true).open(&dst).unwrap();

    let mut region = RegionFile::load(file).unwrap();
    let chunk = region.read_chunk(0, 0, |_| 0).unwrap();

    criterion.bench_with_input(BenchmarkId::new("write_chunk", 1), &1, |b, &_| {
        b.iter(|| {
            region.write_chunk(0, 0, &chunk).unwrap();
        });
    });

    let _ = fs::remove_file(&dst);
}

criterion_group!(benches, write_chunk);
criterion_main!(benches);
