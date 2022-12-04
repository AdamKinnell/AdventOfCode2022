fn value_round(round: &str) -> i32 {
    // Rock = A,X
    // Paper = B,Y
    // Scissors = C,Z
    // [Them] [Us]
    match round {
        "A X" => 3 + 1, // Tie
        "B X" => 6 + 1, // Win
        "C X" => 0 + 1, // Lose
        "A Y" => 0 + 2, // Lose
        "B Y" => 3 + 2, // Tie
        "C Y" => 6 + 2, // Win
        "A Z" => 6 + 3, // Win
        "B Z" => 0 + 3, // Lose
        "C Z" => 3 + 3, // Tie
        &_ => unreachable!()
    }
}

pub fn solve(input: &str) -> i32 {
    input
        .lines()
        .map(|x| value_round(x))
        .sum()
}

pub mod tests {
    use criterion::Criterion;
    use std::path::Path;

    #[test]
    fn verify_example() {
        let input = "A Y\nB X\nC Z";
        assert_eq!(super::solve(input), 15);
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