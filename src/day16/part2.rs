use std::fmt::Debug;
use itertools::Itertools;
use super::lib::{Tunnel, parse_input, find_shortest_distance_to_valves};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
struct Path {
    acc_pressure_release: i32,
    visits: Vec<usize>
}

fn enumerate_all_paths(tunnels_to_valves: &Vec<Vec<Tunnel>>, next_candidates: Vec<usize>, current_path: &mut Vec<usize>, prev_i: usize, acc_pressure_release: i32, time_remaining: i32) -> Vec<Path> {
    if time_remaining < 2 || next_candidates.len() == 0 {
        // !! Early exit !!
        // This path has become terminal - there are no valid candidates to explore
        return [ Path { visits: current_path.clone(), acc_pressure_release } ].to_vec();
    }

    let mut all_subpaths = Vec::new();
    for &next_i in &next_candidates {
        let tunnel = &tunnels_to_valves[prev_i][next_i];
        let time_remaining_after_next_valve = time_remaining - tunnel.distance - 1; // Include 1 turn to open the next valve
        if time_remaining_after_next_valve <= 0 {
            continue; // There isn't enough time for the valve to relieve any pressure once opened
        }

        let pressure_release_from_this_valve = time_remaining_after_next_valve * tunnel.to_valve_flow_rate;
        let acc_pressure_release_at_next = pressure_release_from_this_valve + acc_pressure_release;
        current_path.push(next_i);
        let next_path_candidates = next_candidates.iter().filter(|candidate_i| **candidate_i != next_i).map(|x| *x).collect_vec();
        let mut paths = enumerate_all_paths(
            &tunnels_to_valves,
            next_path_candidates,
            current_path,
            next_i,
            acc_pressure_release_at_next,
            time_remaining_after_next_valve);
        all_subpaths.append(&mut paths);
        current_path.pop();
    }

    all_subpaths.push( Path { visits: current_path.clone(), acc_pressure_release } );

    return all_subpaths;

}

pub fn solve(input: &str) -> i32 {
    let valves = parse_input(input);

    let tunnels_to_valves = (0..valves.len())
        .map(|i| find_shortest_distance_to_valves(&valves, i))
        .collect_vec();

    let useful_valves = valves
        .iter()
        .enumerate()
        .filter_map(|(i, valve)| if &valve.name == "AA" || valve.flow_rate == 0 { return None } else {return Some(i)})
        .collect_vec();

    let start_i = valves.iter().find_position(|valve| valve.name == "AA").unwrap().0;
    let mut paths = enumerate_all_paths(
        &tunnels_to_valves,
        useful_valves.clone(),
        &mut Vec::with_capacity(useful_valves.len()),
        start_i,
        0,
        26);
    paths.sort();
    paths.reverse(); // Largest score first

    // println!("Number of paths: {}", paths.len());
    // for path in &paths {
    //     if path.visits.len() == 2 {
    //         println!("{:?}", path)

    //     }
    // }

    let mut highest_score = 0;
    'a: for a in 0..paths.len() {
        let path_a = &paths[a];
        'b: for b in (a + 1)..paths.len() {
            let path_b = &paths[b];
            let pair_score =  path_a.acc_pressure_release + path_b.acc_pressure_release;

            // Since the paths are sorted by score, future b's for this a will only ever be smaller
            if pair_score < highest_score {
                continue 'a;
            }

            // Ensure the paths don't both touch the same valves
            for a_valve in &path_a.visits {
                if path_b.visits.contains(&a_valve) {
                    // There is an overlap and these paths aren't a valid pair
                    continue 'b;
                }
            }

            // We found candidate pairs
            //println!("A: {:?}, B: {:?}", path_a.visits, path_b.visits);
            highest_score = highest_score.max(pair_score)
        }
    }

    return highest_score;
}

pub mod tests {
    use criterion::Criterion;
    use std::path::Path;

    #[test]
    fn verify_example() {
        assert_eq!(super::solve(super::super::INPUT_EXAMPLE), 1707);
    }

    #[test]
    fn verify_solution() {
        assert_eq!(super::solve(super::super::INPUT), 2504);
    }

    pub fn benchmark(c: &mut Criterion) {
        let path = Path::new(file!());
        let day = path.parent().unwrap().to_str().unwrap();
        let solution = path.file_stem().unwrap().to_str().unwrap();
        let id = day.to_owned() + "_" + solution; // dayX_partY_solveZ
        c.bench_function( &id, |b| b.iter(|| super::solve(super::super::INPUT)));
    }
}