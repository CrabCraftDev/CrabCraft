use crab_world::anvil::region::Region;
use crab_world::blocks::init_blocks;
use crab_world::blocks::BLOCKS;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::path::PathBuf;

pub fn read_chunk(c: &mut Criterion) {
    c.bench_function("chunk", |b| {
        BLOCKS.get_or_init(init_blocks);
        let region = Region::new(PathBuf::from("./r.0.0.mca")).unwrap();

        b.iter(|| {
            let region = region.read_chunk(0, 0, |_| 0).unwrap();
            black_box(region);
        });
    });
}

criterion_group!(benches, read_chunk);
criterion_main!(benches);
