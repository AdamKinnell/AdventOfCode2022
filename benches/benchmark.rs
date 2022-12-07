use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    advent_of_code_2022::day1::part1::tests::benchmark(c);
    advent_of_code_2022::day1::part2::tests::benchmark(c);
    
    advent_of_code_2022::day2::part1_solve1::tests::benchmark(c);
    advent_of_code_2022::day2::part1_solve2::tests::benchmark(c);
    advent_of_code_2022::day2::part2::tests::benchmark(c);

    advent_of_code_2022::day3::part1_solve1::tests::benchmark(c);
    advent_of_code_2022::day3::part1_solve2::tests::benchmark(c);
    advent_of_code_2022::day3::part2::tests::benchmark(c);

    advent_of_code_2022::day4::part1::tests::benchmark(c);
    advent_of_code_2022::day4::part2::tests::benchmark(c);

    advent_of_code_2022::day5::part1_solve1::tests::benchmark(c);
    advent_of_code_2022::day5::part2_solve1::tests::benchmark(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);