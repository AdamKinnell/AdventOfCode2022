use std::collections::HashMap;
use crate::day21::lib::Monkey;
use super::lib::parse_monkeys;

fn evaluate_monkeys(monkey: &str, monkeys: &HashMap<&str, Monkey>) -> i64 {
    let monkey = monkeys.get(monkey).unwrap();

    match monkey {
        Monkey::Operation(o) => {
            let left = evaluate_monkeys(o.left, monkeys);
            let right = evaluate_monkeys(o.right, monkeys);
            o.operation.execute(left, right)
        },
        Monkey::Constant(m) => m.value,
    }
}

pub fn solve(input: &str) -> i64 {
    let monkeys = parse_monkeys(input);
    evaluate_monkeys("root", &monkeys)
}

pub mod tests {
    use criterion::Criterion;
    use std::path::Path;

    #[test]
    fn verify_example() {
        assert_eq!(super::solve(super::super::INPUT_EXAMPLE), 152);
    }

    #[test]
    fn verify_solution() {
        assert_eq!(super::solve(super::super::INPUT), 168502451381566);
    }

    pub fn benchmark(c: &mut Criterion) {
        let path = Path::new(file!());
        let day = path.parent().unwrap().to_str().unwrap();
        let solution = path.file_stem().unwrap().to_str().unwrap();
        let id = day.to_owned() + "_" + solution; // dayX_partY_solveZ
        c.bench_function( &id, |b| b.iter(|| super::solve(super::super::INPUT)));
    }
}