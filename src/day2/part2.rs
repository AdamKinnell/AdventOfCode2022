fn value_round(round: &str) -> i32 {
    // Rock = A
    // Paper = B
    // Scissors = C
    // X = We must lose
    // Y = We must draw
    // Z = We must win
    // "[Their Hand] [Desired Outcome]"
    match round {
        "A X" => 3, // Lose with Scissors
        "B X" => 1, // Lose with Rock
        "C X" => 2, // Lose with Paper
        "A Y" => 3 + 1, // Tie with Rock
        "B Y" => 3 + 2, // Tie with Paper
        "C Y" => 3 + 3, // Tie with Scissors
        "A Z" => 6 + 2, // Win with Paper
        "B Z" => 6 + 3, // Win with Scissors
        "C Z" => 6 + 1, // Win with Rock
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
    fn verify_solution() {
        assert_eq!(super::solve(super::super::INPUT), 12767);
    }

    pub fn benchmark(c: &mut Criterion) {
        let path = Path::new(file!());
        let day = path.parent().unwrap().to_str().unwrap();
        let solution = path.file_stem().unwrap().to_str().unwrap();
        let id = day.to_owned() + "_" + solution; // dayX_partY_solveZ
        c.bench_function( &id, |b| b.iter(|| super::solve(super::super::INPUT)));
    }
}