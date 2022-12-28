use itertools::Itertools;

fn is_any_overlap(line: &&str) -> bool {
    let parsed = line
        .split(|c: char| c == '-' || c == ',')
        .map(|x| x.parse::<u8>().unwrap())
        .collect_tuple()
        .unwrap();
    let (a_min, a_max, b_min, b_max) = parsed;

    // Rule out possibilites where the ranges don't overlap
    if a_max < b_min {return false;}
    if a_min > b_max {return false;}
    if b_max < a_min {return false;}
    if b_min > a_max {return false;}

    true
}

pub fn solve(input: &str) -> i32 {
    return input
        .lines()
        .filter(is_any_overlap)
        .count() as i32;
}

pub mod tests {
    use criterion::Criterion;
    use std::path::Path;

    #[test]
    fn verify_example() {
        assert_eq!(super::solve(super::super::INPUT_EXAMPLE), 4);
    }

    #[test]
    fn verify_solution() {
        assert_eq!(super::solve(super::super::INPUT), 837);
    }

    pub fn benchmark(c: &mut Criterion) {
        let path = Path::new(file!());
        let day = path.parent().unwrap().to_str().unwrap();
        let solution = path.file_stem().unwrap().to_str().unwrap();
        let id = day.to_owned() + "_" + solution; // dayX_partY_solveZ
        c.bench_function( &id, |b| b.iter(|| super::solve(super::super::INPUT)));
    }
}