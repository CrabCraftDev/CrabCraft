use bytes::Bytes;
use crab_registry::REGISTRY;
use crab_world::anvil::region::Region;
use crab_world::blocks::BLOCKS;
use crab_world::blocks::init_blocks;
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use std::fs;

pub fn read_chunk(c: &mut Criterion) {
    c.bench_function("chunk", |b| {
        BLOCKS.get_or_init(init_blocks);
        let region = Region::from(Bytes::from(fs::read("./benches/r.0.0.mca").unwrap()));

        b.iter(|| {
            let region = region.read_chunk(0, 0, |_| 0, REGISTRY.get().unwrap()).unwrap();
            black_box(region);
        });
    });
}

criterion_group!(benches, read_chunk);
criterion_main!(benches);
