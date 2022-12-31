use std::collections::HashMap;

use crate::day21::lib::Monkey;
use super::lib::parse_monkeys;

fn evaluate_monkeys(monkey_name: &str, monkeys: &HashMap<&str, Monkey>) -> (i64, i64) {
    let monkey = monkeys.get(monkey_name).unwrap();

    match monkey {
        Monkey::Operation(o) => {
            let left = evaluate_monkeys(o.left, monkeys).0;
            let right = evaluate_monkeys(o.right, monkeys).0;

            if monkey_name == "root" {
                return (left, right);
            }

            (o.operation.execute(left, right), 0)
        },
        Monkey::Constant(m) => (m.value as i64, 0),
    }
}

// https://shane-o.dev/blog/binary-search-rust-part-2
fn binary_search(start: i64, end: i64, map: &mut impl FnMut(i64) -> i64, find_value: i64, direction: i64) -> Option<i64> {
    let mut lower_bound = start;
    let mut upper_bound = end;

    while lower_bound < upper_bound {
        let middle = (lower_bound + upper_bound) / 2;
        let result = map(middle) * direction;
        //println!("Diff {}", result);
        match result.cmp(&find_value) {
            std::cmp::Ordering::Greater => lower_bound = middle + 1,
            std::cmp::Ordering::Equal => return Some(middle),
            std::cmp::Ordering::Less => upper_bound = middle,
        }

        if upper_bound - lower_bound <= 1 {
            return None;
        }
    }

    None
}

pub fn solve(input: &str) -> i64 {
    let mut monkeys = parse_monkeys(input);

    let mut yell = |x: i64| {
        if let Monkey::Constant(c) = monkeys.get_mut("humn").unwrap() {
            c.value = x;
        }

        let (left, right) = evaluate_monkeys("root", &monkeys);
        left - right
    };

    // Depending on the input, we may need to search ascending or descending
    let mut number_to_yell = binary_search(-(2i64.pow(50)), 2i64.pow(50), &mut yell, 0, 1);
    if number_to_yell.is_none() {
        number_to_yell = binary_search(-(2i64.pow(50)), 2i64.pow(50), &mut yell, 0, -1);
    }

    // We need the lowest possible value to yell
    let mut last = number_to_yell.unwrap();
    loop {
        let diff = yell(last - 1);
        if diff == 0 {
            last -= 1;
        } else {
            return last;
        }
    }
}

pub mod tests {
    use criterion::Criterion;
    use std::path::Path;

    #[test]
    fn verify_example() {
        assert_eq!(super::solve(super::super::INPUT_EXAMPLE), 301);
    }

    #[test]
    fn verify_solution() {
        assert_eq!(super::solve(super::super::INPUT), 3343167719435);
    }

    pub fn benchmark(c: &mut Criterion) {
        let path = Path::new(file!());
        let day = path.parent().unwrap().to_str().unwrap();
        let solution = path.file_stem().unwrap().to_str().unwrap();
        let id = day.to_owned() + "_" + solution; // dayX_partY_solveZ
        c.bench_function( &id, |b| b.iter(|| super::solve(super::super::INPUT)));
    }
}