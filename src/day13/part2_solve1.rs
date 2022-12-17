use std::cmp::Ordering;

use itertools::Itertools;

use super::lib::{ComparisonResult, compare_data};

fn find_decoder_key(packets: &Vec<&str>) -> usize {
    let mut decoder_key = 1;
    for (i, packet) in packets.iter().enumerate() {
        if packet == &"[[2]]" || packet == &"[[6]]" {
            decoder_key *= i + 1;
            if decoder_key != i + 1 {
                // We found the second packet
                return decoder_key;
            }
        }
    }
    unreachable!()
}

pub fn solve(input: &str) -> usize {
    let mut packets = input
    .split("\r\n")
    .filter(|x| !x.is_empty())
    .chain(["[[2]]", "[[6]]"])
    .collect_vec();

    packets.sort_unstable_by(|left, right| {
        match compare_data(left, right) {
            ComparisonResult::CorrectOrder => Ordering::Less,
            ComparisonResult::IncorrectOrder => Ordering::Greater,
            ComparisonResult::Indeterminate => return Ordering::Equal,
        }
    });
    //packets.iter().for_each(|x| println!("{}",x));

    return find_decoder_key(&packets);
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