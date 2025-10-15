use crab_registry::{REGISTRY, RegistryManager};
use crab_world::anvil::region::file::RegionFile;
use crab_world::blocks::BLOCKS;
use crab_world::blocks::init_blocks;
use criterion::{Criterion, criterion_group, criterion_main};
use std::fs::File;

pub fn count_blocks(criterion: &mut Criterion) {
    criterion.bench_function("count_non_air_blocks", |b| {
        BLOCKS.get_or_init(init_blocks);
        REGISTRY.get_or_init(RegistryManager::default);

        let mut region = RegionFile::load(File::open("./tests/data/r.0.0.mca").unwrap()).unwrap();
        let mut chunk = region.read_chunk(0, 0, |_| 0, REGISTRY.get().unwrap()).unwrap();

        b.iter(|| {
            for section in &mut chunk.sections {
                section.count_non_air_blocks();
            }
        });
    });
}

criterion_group!(benches, count_blocks);
criterion_main!(benches);
