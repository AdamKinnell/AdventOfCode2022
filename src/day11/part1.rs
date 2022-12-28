use std::vec;
use itertools::Itertools;
use super::lib::*;

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
                    if worry_level % monkey.divisor == 0 {
                        (monkey.on_success, worry_level)
                    } else {
                        (monkey.on_failure, worry_level)
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