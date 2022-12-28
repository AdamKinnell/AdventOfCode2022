fn value_round(round: &str) -> i32 {
    // Rock = A,X
    // Paper = B,Y
    // Scissors = C,Z
    // [Them] [Us]
    match round {
        "A X" => 3 + 1, // Tie
        "B X" => 1, // Lose
        "C X" => 6 + 1, // Win
        "A Y" => 6 + 2, // Win
        "B Y" => 3 + 2, // Tie
        "C Y" => 2, // Lose
        "A Z" => 3, // Lose
        "B Z" => 6 + 3, // Win
        "C Z" => 3 + 3, // Tie
        &_ => unreachable!()
    }
}

pub fn solve(input: &str) -> i32 {
    input
        .lines()
        .map(value_round)
        .sum()
}

pub mod tests {
    use criterion::Criterion;
    use std::path::Path;

    #[test]
    fn verify_example() {
        assert_eq!(super::solve(super::super::INPUT_EXAMPLE), 15);
    }

    #[test]
    fn verify_solution() {
        assert_eq!(super::solve(super::super::INPUT), 11666);
    }

    pub fn benchmark(c: &mut Criterion) {
        let path = Path::new(file!());
        let day = path.parent().unwrap().to_str().unwrap();
        let solution = path.file_stem().unwrap().to_str().unwrap();
        let id = day.to_owned() + "_" + solution; // dayX_partY_solveZ
        c.bench_function( &id, |b| b.iter(|| super::solve(super::super::INPUT)));
    }
}