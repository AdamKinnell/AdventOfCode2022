use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day1_part1", |b| b.iter(||  advent_of_code_2022::day1::part1::tests::benchmark()));
    c.bench_function("day1_part2", |b| b.iter(||  advent_of_code_2022::day1::part2::tests::benchmark()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);