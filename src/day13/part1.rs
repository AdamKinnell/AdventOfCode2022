use super::lib::{ComparisonResult, compare_data};

pub fn solve(input: &str) -> usize {
    let pairs = input.split("\r\n\r\n");
    pairs
        .enumerate()
        .filter(|(_, pair)| {
            let (left, right) = pair.split_once("\r\n").unwrap();
            compare_data(left, right) == ComparisonResult::CorrectOrder
        })
        .map(|(i,_)| i + 1)
        .sum()
}

pub mod tests {
    use criterion::Criterion;
    use std::path::Path;


    #[test]
    fn verify_check_pairs() {
        assert_eq!(super::compare_data("[1,1,3,1,1]", "[1,1,5,1,1]"), super::ComparisonResult::CorrectOrder);
        assert_eq!(super::compare_data("[[1],[2,3,4]]", "[[1],4]"), super::ComparisonResult::CorrectOrder);
        assert_eq!(super::compare_data("[9]", "[[8,7,6]]"), super::ComparisonResult::IncorrectOrder);
        assert_eq!(super::compare_data("[[4,4],4,4]", "[[4,4],4,4,4]"), super::ComparisonResult::CorrectOrder);
        assert_eq!(super::compare_data("[7,7,7,7]", "[7,7,7]"), super::ComparisonResult::IncorrectOrder);
        assert_eq!(super::compare_data("[]", "[3]"), super::ComparisonResult::CorrectOrder);
        assert_eq!(super::compare_data("[[[]]]", "[[]]"), super::ComparisonResult::IncorrectOrder);
        assert_eq!(super::compare_data("[1,[2,[3,[4,[5,6,7]]]],8,9]", "[1,[2,[3,[4,[5,6,0]]]],8,9]"), super::ComparisonResult::IncorrectOrder);
    }

    #[test]
    fn verify_example() {
        assert_eq!(super::solve(super::super::INPUT_EXAMPLE), 13);
    }

    #[test]
    fn verify_solution() {
        assert_eq!(super::solve(super::super::INPUT), 5503);
    }

    pub fn benchmark(c: &mut Criterion) {
        let path = Path::new(file!());
        let day = path.parent().unwrap().to_str().unwrap();
        let solution = path.file_stem().unwrap().to_str().unwrap();
        let id = day.to_owned() + "_" + solution; // dayX_partY_solveZ
        c.bench_function( &id, |b| b.iter(|| super::solve(super::super::INPUT)));
    }
}