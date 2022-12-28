use itertools::Itertools;
use super::lib::{Tunnel, parse_input, find_shortest_distance_to_valves};

fn find_highest_pressure_release(tunnels_to_valves: &Vec<Vec<Tunnel>>, next_candidates: Vec<usize>, prev_i: usize, time_remaining: i32) -> i32 {
    if time_remaining < 2 {
        return 0; // There isn't enough time to move to another valve (>= 1 minute) and open it (1 minute)
    }

    let mut highest_pressure_released = 0;
    for &next_i in &next_candidates {
        let tunnel = &tunnels_to_valves[prev_i][next_i];
        let time_remaining_after_next_valve = time_remaining - tunnel.distance - 1; // Include 1 turn to open the next valve
        if time_remaining_after_next_valve <= 0 {
            continue; // There isn't enough time for the valve to relieve any pressure once opened
        }

        let next_path_candidates = next_candidates.iter().filter(|candidate_i| **candidate_i != next_i).copied().collect_vec();
        let pressure_release_from_this_valve = time_remaining_after_next_valve * tunnel.to_valve_flow_rate;

        let pressure_released_from_remaining_path = find_highest_pressure_release( tunnels_to_valves, next_path_candidates, next_i, time_remaining_after_next_valve);
        let pressure_released = pressure_release_from_this_valve + pressure_released_from_remaining_path;
        highest_pressure_released = highest_pressure_released.max(pressure_released)
    }

    highest_pressure_released
}

pub fn solve(input: &str) -> i32 {
    let valves = parse_input(input);

    let tunnels_to_valves = (0..valves.len())
        .map(|i| find_shortest_distance_to_valves(&valves, i))
        .collect_vec();

    let useful_valves = valves
        .iter()
        .enumerate()
        .filter_map(|(i, valve)| if &valve.name == "AA" || valve.flow_rate == 0 { None } else {Some(i)})
        .collect_vec();

    let start_i = valves.iter().find_position(|valve| valve.name == "AA").unwrap().0;
    

    find_highest_pressure_release( &tunnels_to_valves, useful_valves, start_i, 30)
}

pub mod tests {
    use criterion::Criterion;
    use std::path::Path;

    #[test]
    fn verify_example() {
        assert_eq!(super::solve(super::super::INPUT_EXAMPLE), 1651);
    }

    #[test]
    fn verify_solution() {
        assert_eq!(super::solve(super::super::INPUT), 1716);
    }

    pub fn benchmark(c: &mut Criterion) {
        let path = Path::new(file!());
        let day = path.parent().unwrap().to_str().unwrap();
        let solution = path.file_stem().unwrap().to_str().unwrap();
        let id = day.to_owned() + "_" + solution; // dayX_partY_solveZ
        c.bench_function( &id, |b| b.iter(|| super::solve(super::super::INPUT)));
    }
}