use criterion::{black_box, criterion_group, criterion_main, Criterion};
use aoc2020::days::*;

fn criterion_benchmark(c: &mut Criterion) {

    let mut entries = lines_from!("1a", u32);
    entries.sort();

    c.bench_function("day1e", |b| b.iter(|| day1e()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);