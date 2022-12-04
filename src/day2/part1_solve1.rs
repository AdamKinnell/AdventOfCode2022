use itertools::Itertools;

enum Hand {
    Rock,
    Paper,
    Scissors
}

enum Result {
    Win,
    Tie,
    Lose
}

impl Result {
    fn get_score_value(&self) -> i32 {
        match self {
            Result::Win => 6,
            Result::Tie => 3,
            Result::Lose => 0,
        }
    }
}

impl Hand {

    fn from_string(hand: &str) -> Hand {
        match hand {
            "A" | "X" => Hand::Rock,
            "B" | "Y" => Hand::Paper,
            "C" | "Z" => Hand::Scissors,
            &_ => unreachable!()
        }
    }

    fn play_against(&self, other: &Hand) -> Result {
        match (self, other) {
            (Hand::Rock, Hand::Scissors) |
            (Hand::Paper, Hand::Rock) |
            (Hand::Scissors, Hand::Paper) => Result::Win,
            (Hand::Rock, Hand::Rock) |
            (Hand::Paper, Hand::Paper) |
            (Hand::Scissors, Hand::Scissors) => Result::Tie,
            (Hand::Rock, Hand::Paper) |
            (Hand::Paper, Hand::Scissors) |
            (Hand::Scissors, Hand::Rock) => Result::Lose,
        }
    }

    fn get_score_value(&self) -> i32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }
}

fn value_round(round: &str) -> i32 {
    let mut round = round.split_ascii_whitespace();

    let them = Hand::from_string(round.next().unwrap());
    let me = Hand::from_string(round.next().unwrap());

    let result = me.play_against(&them);
    return me.get_score_value() + result.get_score_value();
}

pub fn solve(input: &str) -> i32 {
    let rounds = input.lines().collect_vec();
    return rounds.iter()
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