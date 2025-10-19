use crab_registry::{REGISTRY, RootRegistry};
use crab_world::anvil::region::file::RegionFile;
use crab_world::blocks::block_registry::BlockInformation;
use crab_world::blocks::init_blocks;
use criterion::{Criterion, criterion_group, criterion_main};
use std::fs::File;
use std::sync::Arc;

pub fn count_blocks(criterion: &mut Criterion) {
    criterion.bench_function("count_non_air_blocks", |b| {
        let mut registry = RootRegistry::new();
        registry.register_registry(BlockInformation::REGISTRY, init_blocks());
        REGISTRY.store(Arc::new(registry));

        let mut region = RegionFile::load(File::open("./tests/data/r.0.0.mca").unwrap()).unwrap();
        let mut chunk = region.read_chunk(0, 0, |_| 0).unwrap();

        b.iter(|| {
            for section in &mut chunk.sections {
                section.count_non_air_blocks();
            }
        });
    });
}

criterion_group!(benches, count_blocks);
criterion_main!(benches);
