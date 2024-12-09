use bytes::BytesMut;
use crab_world::anvil::region::Region;
use crab_world::blocks::init_blocks;
use crab_world::blocks::BLOCKS;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs::{self};

pub fn read_chunk(c: &mut Criterion) {
    c.bench_function("chunk", |b| {
        BLOCKS.get_or_init(init_blocks);

        let bytes = BytesMut::from_iter(fs::read("./r.0.0.mca").unwrap());
        let region = Region::new(bytes);

        b.iter(|| {
            let region = region.read_chunk(0, 0, |_| 0).unwrap();
            black_box(region);
        });
    });
}

criterion_group!(benches, read_chunk);
criterion_main!(benches);
