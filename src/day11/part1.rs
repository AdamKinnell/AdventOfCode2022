use std::vec;
use itertools::Itertools;

pub struct Monkey<'a> {
    items: Vec<usize>,
    operation: &'a dyn Fn(usize) -> usize,
    test: &'a dyn Fn(usize) -> bool,
    on_success: usize,
    on_failure: usize
}

pub fn get_example_monkeys () -> Vec<Monkey<'static>> {
    let m0 = Monkey {
        items: vec!(79, 98),
        operation: &|x| x * 19,
        test: &|x| x % 23 == 0,
        on_success: 2,
        on_failure: 3,
    };

    let m1 = Monkey {
        items: vec!(54, 65, 75, 74),
        operation: &|x| x + 6,
        test: &|x| x % 19 == 0,
        on_success: 2,
        on_failure: 0,
    };

    let m2 = Monkey {
        items: vec!(79, 60, 97),
        operation: &|x| x * x,
        test: &|x| x % 13 == 0,
        on_success: 1,
        on_failure: 3,
    };

    let m3 = Monkey {
        items: vec!(74),
        operation: &|x| x + 3,
        test: &|x| x % 17 == 0,
        on_success: 0,
        on_failure: 1,
    };

    return vec![m0, m1, m2, m3];
}

fn get_actual_monkeys() -> Vec<Monkey<'static>> {
    let m0 = Monkey {
        items: vec!(83, 88, 96, 79, 86, 88, 70),
        operation: &|x| x * 5,
        test: &|x| x % 11 == 0,
        on_success: 2,
        on_failure: 3,
    };

    let m1 = Monkey {
        items: vec!(59, 63, 98, 85, 68, 72),
        operation: &|x| x * 11,
        test: &|x| x % 5 == 0,
        on_success: 4,
        on_failure: 0,
    };

    let m2 = Monkey {
        items: vec!(90, 79, 97, 52, 90, 94, 71, 70),
        operation: &|x| x + 2,
        test: &|x| x % 19 == 0,
        on_success: 5,
        on_failure: 6,
    };

    let m3 = Monkey {
        items: vec!(97, 55, 62),
        operation: &|x| x + 5,
        test: &|x| x % 13 == 0,
        on_success: 2,
        on_failure: 6,
    };

    let m4 = Monkey {
        items: vec!(74, 54, 94, 76),
        operation: &|x| x * x,
        test: &|x| x % 7 == 0,
        on_success: 0,
        on_failure: 3,
    };

    let m5 = Monkey {
        items: vec!(58),
        operation: &|x| x + 4,
        test: &|x| x % 17 == 0,
        on_success: 7,
        on_failure: 1,
    };

    let m6 = Monkey {
        items: vec!(66, 63),
        operation: &|x| x + 6,
        test: &|x| x % 2 == 0,
        on_success: 7,
        on_failure: 5,
    };

    let m7 = Monkey {
        items: vec!(56, 56, 90, 96, 68),
        operation: &|x| x + 7,
        test: &|x| x % 3 == 0,
        on_success: 4,
        on_failure: 1,
    };

    return vec![m0, m1, m2, m3, m4, m5, m6, m7];
}

fn solve(mut monkeys: Vec<Monkey>) -> usize {
    let mut num_items_inspected = vec![0; monkeys.len()];

    for _round in 0..20 {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            let inspection_result = monkey.items
                .drain(..)
                .map(|item| {
                    // Inspect
                    num_items_inspected[i] += 1;
                    let mut worry_level = (monkey.operation)(item);
                    // Cooldown
                    worry_level /= 3;
                    // Test
                    if (monkey.test)(worry_level) {
                        return (monkey.on_success, worry_level);
                    } else {
                        return (monkey.on_failure, worry_level);
                    }
                }).collect_vec();

            //num_items_inspected[i] += inspection_result.len();
            inspection_result.iter().for_each(|(monkey_index, item)| {
                monkeys[*monkey_index].items.push(*item)
            })
        }

        // println!("\nRound {}", _round + 1);
        // monkeys.iter().enumerate().for_each(|(i, m)| {
        //     println!("{i}: {:?}", (i, &m.items))
        // });
    }

    num_items_inspected.sort();
    return num_items_inspected.iter().rev().take(2).product();
}

pub mod tests {
    use criterion::Criterion;
    use std::path::Path;

    #[test]
    fn verify_example() {
        assert_eq!(super::solve(super::get_example_monkeys()), 10605);
    }

    #[test]
    fn verify_solution() {
        assert_eq!(super::solve(super::get_actual_monkeys()), 64032);
    }

    pub fn benchmark(c: &mut Criterion) {
        let path = Path::new(file!());
        let day = path.parent().unwrap().to_str().unwrap();
        let solution = path.file_stem().unwrap().to_str().unwrap();
        let id = day.to_owned() + "_" + solution; // dayX_partY_solveZ
        c.bench_function( &id, |b| b.iter(|| super::solve(super::get_actual_monkeys())));
    }
}