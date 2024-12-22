use bytes::Bytes;
use crab_registry::{RegistryManager, REGISTRY};
use crab_world::anvil::region::Region;
use crab_world::blocks::init_blocks;
use crab_world::blocks::BLOCKS;
use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;

pub fn count_blocks(c: &mut Criterion) {
    c.bench_function("count_non_air_blocks", |b| {
        BLOCKS.get_or_init(init_blocks);
        REGISTRY.get_or_init(RegistryManager::default);

        let region = Region::from(Bytes::from(fs::read("./benches/r.0.0.mca").unwrap()));
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
