use std::{vec, hash::{Hash, Hasher}, collections::{hash_map::DefaultHasher, HashSet, HashMap}};
use itertools::Itertools;
use super::lib::*;

fn hash_monkey_items(monkeys: &Vec<Monkey>) -> u64 {
    let mut hasher = DefaultHasher::new();
    monkeys.iter().for_each(|monkey| monkey.items.hash(&mut hasher));
    return hasher.finish();
}

fn find_top_two(items: &Vec<usize>) -> (usize, usize) {
    let mut largest_a  = 0;
    let mut largest_b  = 0;

    for item in items {
        if *item > largest_a {
            largest_b = largest_a;
            largest_a = *item;
        } else if *item > largest_b {
            largest_b = *item;
            
        }
    }

    return (largest_a, largest_b);
}

fn solve(mut monkeys: Vec<Monkey>) -> usize {
    let mut num_items_inspected = vec![0; monkeys.len()];
    let mut top_two_per_round = Vec::new();
    let mut seen_states = HashMap::new();
    let worry_lcm: usize = monkeys.iter().map(|m| m.divisor).product();

    let mut round = 0;
    let total_rounds = 10000;
    while round < total_rounds {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            let inspection_result = monkey.items
                .drain(..)
                .map(|item| {
                    // Inspect
                    let mut worry_level = (monkey.operation)(item);
                    // Keep size in check
                    worry_level %= worry_lcm;
                    // Test
                    if worry_level % monkey.divisor == 0 {
                        return (monkey.on_success, worry_level);
                    } else {
                        return (monkey.on_failure, worry_level);
                    }
                }).collect_vec();

            // Throw items to other monkeys
            num_items_inspected[i] += inspection_result.len();
            inspection_result.iter().for_each(|(monkey_index, item)| {
                monkeys[*monkey_index].items.push(*item)
            });
        }

        // Check if we've seen this state before after a round
        let hash = hash_monkey_items(&monkeys);
        let this_top_two = find_top_two(&num_items_inspected);
        if let Some(last_seen_round) = seen_states.insert(hash, round) {

            let rounds_per_cycle = round - last_seen_round;
            let remaining_rounds = total_rounds - round;
            let remaining_full_cycles = remaining_rounds / rounds_per_cycle; // Rounded down
            let remainder_rounds_in_partial_cycle = remaining_rounds % rounds_per_cycle; // Remainder

            // Calculate expected inspections by top-two monkeys after skipping the remaining full cycles
            let (last_t1, last_t2) = top_two_per_round[last_seen_round];
            let (this_t1, this_t2) = this_top_two;
            let t1 = this_t1 + (this_t1 - last_t1) * remaining_full_cycles;
            let t2 = this_t2 + (this_t2 - last_t2) * remaining_full_cycles;

            // Calculate expected inspections by top-two monkeys after skipping the remaining rounds of a partial cycle
            let final_round_in_last_cycle = last_seen_round + remainder_rounds_in_partial_cycle;
            let (last_part_t1, last_part_t2) = top_two_per_round[final_round_in_last_cycle - 1];
            let t1 = t1 + (last_part_t1 - last_t1);
            let t2 = t2 + (last_part_t2 - last_t2);

            return t1 * t2;
        }
        top_two_per_round.push(this_top_two);

        // println!("\nRound {}", round + 1);
        // monkeys.iter().enumerate().for_each(|(i, m)| {
        //     println!("{i}: {:?}", (i, &m.items))
        // });

        round += 1
    }

    let (t1, t2) = find_top_two(&num_items_inspected);
    return t1 * t2;
}

pub mod tests {
    use criterion::Criterion;
    use std::path::Path;

    #[test]
    fn verify_example() {
        assert_eq!(super::solve(super::get_example_monkeys()), 2713310158);
    }

    #[test]
    fn verify_solution() {
        assert_eq!(super::solve(super::get_actual_monkeys()), 12729522272);
    }

    pub fn benchmark(c: &mut Criterion) {
        let path = Path::new(file!());
        let day = path.parent().unwrap().to_str().unwrap();
        let solution = path.file_stem().unwrap().to_str().unwrap();
        let id = day.to_owned() + "_" + solution; // dayX_partY_solveZ
        c.bench_function( &id, |b| b.iter(|| super::solve(super::get_actual_monkeys())));
    }
}