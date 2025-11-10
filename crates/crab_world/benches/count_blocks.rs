use crab_world::anvil::region::file::RegionFile;
use criterion::{Criterion, criterion_group, criterion_main};
use std::fs::File;

pub fn count_blocks(criterion: &mut Criterion) {
    criterion.bench_function("count_non_air_blocks", |b| {
        let mut region = RegionFile::load(File::open("./tests/data/r.0.0.mca").unwrap()).unwrap();
        let mut chunk = region.read_chunk(0, 0).unwrap();

        b.iter(|| {
            for section in &mut chunk.sections {
                section.count_non_air_blocks();
            }
        });
    });
}

criterion_group!(benches, count_blocks);
criterion_main!(benches);
