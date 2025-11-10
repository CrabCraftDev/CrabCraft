use crab_world::anvil::region::file::RegionFile;
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use std::fs::File;

pub fn read_chunk(criterion: &mut Criterion) {
    let mut region = RegionFile::load(File::open("./tests/data/r.0.0.mca").unwrap()).unwrap();

    criterion.bench_with_input(BenchmarkId::new("read_chunk", 1), &1, |b, &_| {
        b.iter(|| {
            region.read_chunk(0, 0).unwrap();
        });
    });

    criterion.bench_with_input(BenchmarkId::new("read_chunk", 64), &64, |b, &_| {
        b.iter(|| {
            for x in 0..8 {
                for z in 0..8 {
                    region.read_chunk(x, z).unwrap();
                }
            }
        });
    });
}

criterion_group!(benches, read_chunk);
criterion_main!(benches);
