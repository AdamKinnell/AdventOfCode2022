use std::{collections::{BinaryHeap}, cmp::Ordering};

use itertools::Itertools;
use regex::Regex;

struct Tunnel {
    distance: i32,
    to_valve_index: usize
}

struct Valve {
    name: String,
    flow_rate: i32,
    links: Vec<Tunnel>
}

// From https://doc.rust-lang.org/std/collections/binary_heap/index.html#examples
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    distance: i32,
    valve_i: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
            .then_with(|| self.valve_i.cmp(&other.valve_i))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_shortest_distance_to_valves<'a>(valves: &'a Vec<Valve>, start_valve_i: usize) -> Vec<Tunnel> {
    let mut visitation_queue = BinaryHeap::new();
    let mut distance_from_start = (0..valves.len()).map(|_| i32::MAX).collect_vec();

    // Find shortest path to all valves
    visitation_queue.push(State { distance: 0, valve_i: start_valve_i });
    distance_from_start[start_valve_i] = 0;
    while let Some(State { distance, valve_i}) = visitation_queue.pop() {
        if distance > distance_from_start[valve_i] {
            continue; // Already found a better path to this node
        }

        for tunnel in &valves[valve_i].links {
            let next = State { distance: distance + tunnel.distance, valve_i: tunnel.to_valve_index };
            if next.distance < distance_from_start[next.valve_i] {
                visitation_queue.push(next);
                distance_from_start[next.valve_i] = next.distance;
            }
        }
    }

    // Return shortest paths
    return distance_from_start
        .iter()
        .filter(|distance| **distance != i32::MAX)
        .enumerate()
        .map(|(i, distance)| Tunnel { distance: *distance, to_valve_index: i } )
        .collect_vec()
}

fn parse_input(input: &str) -> Vec<Valve> {
    let regex = r"^Valve (..) has flow rate=(\d+); tunnels? leads? to valves? (.+)$";
    let re = Regex::new(regex).unwrap();

    // Parse valves
    let mut valves = input
        .lines()
        .map(|line| {
            let captures = re.captures(line).unwrap();
            let name = captures[1].to_owned();
            let flow_rate: i32 = captures[2].parse().unwrap();
            let connections = captures[3].split(", ").map(str::to_string).collect_vec();
            return (name, flow_rate, connections);
        }).collect_vec();

    valves.sort_by_key(|(name,flow,_)| if name == "AA" {1} else {*flow});
    valves.reverse(); // Sort useless valves to the end so we can remove them later without breaking our index references

    // Convert to more efficient index-based connections
    let find_valve_index = |name: &str| valves.iter().find_position(|(n,_,_)| name == n).unwrap().0;
    let indexed_valves = valves
        .iter()
        .map(|(name, flow, connections)|
            Valve {
                name: name.to_string(),
                flow_rate: *flow,
                links: connections
                    .iter()
                    .map(|name| Tunnel { distance: 1, to_valve_index: find_valve_index(name.as_str()) })
                    .collect_vec()
            })
        .collect_vec();

    return indexed_valves;

}

fn find_highest_pressure_release(valves: &Vec<Valve>, tunnels_to_valves: &Vec<Vec<Tunnel>>, path_candidates: Vec<usize>, prev_i: usize, time_remaining: i32) -> i32 {
    if time_remaining <= 1 {
        return 0; // There isn't enough time to move to another valve (1) and open it (1)
    }

    let mut highest_pressure_released = 0;
    for &next_i in &path_candidates {
        let tunnel = &tunnels_to_valves[prev_i][next_i];
        let time_remaining_after_next = time_remaining - tunnel.distance - 1; // Include 1 turn to open the next valve
        if time_remaining_after_next <= 0 {
            continue; // There isn't enough time for the valve to relieve any pressure once opened
        }

        let next_path_candidates = path_candidates.iter().filter(|candidate_i| **candidate_i != next_i).map(|x| *x).collect_vec();
        let pressure_release_from_this_valve = time_remaining_after_next * valves[next_i].flow_rate;

        let pressure_released_from_remaining_path = find_highest_pressure_release(&valves, &tunnels_to_valves, next_path_candidates, next_i, time_remaining_after_next);
        let pressure_released = pressure_release_from_this_valve + pressure_released_from_remaining_path;
        highest_pressure_released = highest_pressure_released.max(pressure_released)
    }

    return highest_pressure_released;
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
    let largest_pressure_released = find_highest_pressure_release(&valves, &tunnels_to_valves, useful_valves, start_i, 30);

    return largest_pressure_released;
}

pub mod tests {
    use criterion::Criterion;
    use std::path::Path;

    #[test]
    fn verify_example() {
        assert_eq!(super::solve(super::super::INPUT_EXAMPLE), 1716);
    }

    #[test]
    fn verify_solution() {
        assert_eq!(super::solve(super::super::INPUT), -1);
    }

    pub fn benchmark(c: &mut Criterion) {
        let path = Path::new(file!());
        let day = path.parent().unwrap().to_str().unwrap();
        let solution = path.file_stem().unwrap().to_str().unwrap();
        let id = day.to_owned() + "_" + solution; // dayX_partY_solveZ
        c.bench_function( &id, |b| b.iter(|| super::solve(super::super::INPUT)));
    }
}