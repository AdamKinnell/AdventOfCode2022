use itertools::Itertools;

fn is_fully_contained(line: &&str) -> bool {
    let parsed = line
        .split(|c: char| c == '-' || c == ',')
        .map(|x| x.parse::<u8>().unwrap())
        .collect_tuple()
        .unwrap();
    let (a_min, a_max, b_min, b_max) = parsed;

    let a_in_b = a_min >= b_min && a_max <= b_max;
    let b_in_a = b_min >= a_min && b_max <= a_max;

    a_in_b || b_in_a
}

pub fn solve(input: &str) -> i32 {
    return input
        .lines()
        .filter(is_fully_contained)
        .count() as i32;
}

pub mod tests {
    use criterion::Criterion;
    use std::path::Path;

    #[test]
    fn verify_example() {
        assert_eq!(super::solve(super::super::INPUT_EXAMPLE), 2);
    }

    #[test]
    fn verify_solution() {
        assert_eq!(super::solve(super::super::INPUT), 450);
    }

    pub fn benchmark(c: &mut Criterion) {
        let path = Path::new(file!());
        let day = path.parent().unwrap().to_str().unwrap();
        let solution = path.file_stem().unwrap().to_str().unwrap();
        let id = day.to_owned() + "_" + solution; // dayX_partY_solveZ
        c.bench_function( &id, |b| b.iter(|| super::solve(super::super::INPUT)));
    }
}