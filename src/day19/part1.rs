use crate::day19::lib::{SimulationState, calculate_blueprint_quality};
use super::lib::parse_blueprints;

pub fn solve(input: &str) -> i32 {
    let blueprints = parse_blueprints(input);

    blueprints
        .enumerate()
        .map(|(i, blueprint)| {
            let quality = calculate_blueprint_quality(&blueprint, SimulationState::new(), 25);
            println!("Blueprint {} resulted in {} geodes", i + 1, quality);
            (i as i32+1) * quality as i32
        })
        .sum()
}

pub mod tests {
    use criterion::Criterion;
    use std::path::Path;

    #[test]
    fn verify_example() {
        assert_eq!(super::solve(super::super::INPUT_EXAMPLE), 33);
    }

    #[test]
    fn verify_solution() {
        assert_eq!(super::solve(super::super::INPUT), 1962);
    }

    pub fn benchmark(c: &mut Criterion) {
        let path = Path::new(file!());
        let day = path.parent().unwrap().to_str().unwrap();
        let solution = path.file_stem().unwrap().to_str().unwrap();
        let id = day.to_owned() + "_" + solution; // dayX_partY_solveZ
        let mut group = c.benchmark_group("day19_part1");
        group.sample_size(10);
        group.bench_function( &id, |b| b.iter(|| super::solve(super::super::INPUT)));
    }
}