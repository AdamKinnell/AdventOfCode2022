use itertools::Itertools;
use super::lib::{ComparisonResult, compare_data};

pub fn solve(input: &str) -> usize {
    let packets = input
    .split("\r\n")
    .filter(|x| !x.is_empty())
    .collect_vec();

    // Find index of each divider packet
    let two_index = packets.iter().filter(|packet| {
        return compare_data(packet, "[[2]]") == ComparisonResult::CorrectOrder
    }).count() + 1;

    let six_index = packets.iter().filter(|packet| {
        return compare_data(packet, "[[6]]") == ComparisonResult::CorrectOrder
    }).count() + 1;

    // Account for the two divider packets not actually being in the input
    if two_index < six_index {
        return two_index * (six_index + 1);
    } else {
        return (two_index + 1) * six_index;
    }
}

pub mod tests {
    use criterion::Criterion;
    use std::path::Path;

    #[test]
    fn verify_example() {
        assert_eq!(super::solve(super::super::INPUT_EXAMPLE), 140);
    }

    #[test]
    fn verify_solution() {
        assert_eq!(super::solve(super::super::INPUT), 20952);
    }

    pub fn benchmark(c: &mut Criterion) {
        let path = Path::new(file!());
        let day = path.parent().unwrap().to_str().unwrap();
        let solution = path.file_stem().unwrap().to_str().unwrap();
        let id = day.to_owned() + "_" + solution; // dayX_partY_solveZ
        c.bench_function( &id, |b| b.iter(|| super::solve(super::super::INPUT)));
    }
}