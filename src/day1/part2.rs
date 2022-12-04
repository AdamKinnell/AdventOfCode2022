use itertools::Itertools;

pub fn solve1(input: &str) -> i32 {
    let mut elves: Vec<i32> = input.split("\r\n\r\n")
        .map(|elf| {
            elf
            .lines()
            .filter(|x| !x.is_empty())
            .map(|line| line.parse::<i32>().unwrap())
            .sum()
        }).collect();
    
    elves.sort();

    return elves.iter().rev().take(3).sum();
}

pub fn solve2(input: &str) -> i32 {
    let elves = input
        .lines()
        .map(|l| l.parse::<i32>())
        .coalesce(|prev, curr| {
            match (&prev, &curr) {
                (Ok(x), Ok(y)) => Ok(Ok(x + y)),
                (Ok(_), Err(_)) => Err((prev, curr)),
                (Err(_), Ok(y)) => Ok(Ok(*y)),
                (Err(_), Err(_)) => Err((prev, curr)),
            }
        }).map(|x| x.unwrap());

    return elves.sorted().rev().take(3).sum();
}

pub mod tests {
    use criterion::Criterion;

    #[test]
    fn verify_solution() {
        assert_eq!(super::solve1(super::super::INPUT), 195625);
        assert_eq!(super::solve2(super::super::INPUT), 195625);
    }

    pub fn benchmark(c: &mut Criterion) {
        c.bench_function("day1_part2_solve1", |b| b.iter(|| super::solve1(super::super::INPUT)));
        c.bench_function("day1_part2_solve2", |b| b.iter(|| super::solve2(super::super::INPUT)));
    }
}