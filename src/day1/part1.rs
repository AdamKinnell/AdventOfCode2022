
pub fn solve(input: &str) -> i32 {
    let mut iter = input.lines();
    let mut largest = 0;
    loop {
        let elf: i32 = iter.by_ref()
            .take_while(|line| !line.is_empty())
            .map(|line| line.parse::<i32>().unwrap())
            .sum();

        if elf == 0 {
            break;
        }

        largest = std::cmp::max(largest, elf);
    }
    return largest;
}


pub mod tests {
    use criterion::Criterion;

    #[test]
    fn verify_example() {
        assert_eq!(super::solve(super::super::INPUT_EXAMPLE), 24000);
    }

    #[test]
    fn verify_solution() {
        assert_eq!(super::solve(super::super::INPUT), 65912);
    }

    pub fn benchmark(c: &mut Criterion) {
        c.bench_function("day1_part1_solve1", |b| b.iter(|| super::solve(super::super::INPUT)));
    }
}