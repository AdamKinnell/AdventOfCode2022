use std::{collections::{BinaryHeap}, cmp::Ordering};

use itertools::Itertools;
use num::Unsigned;
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

fn find_paths_to_useful_valves<'a>(valves: &'a Vec<Valve>, start_valve: &'a Valve, path: &mut Vec<&'a str>) -> Vec<Tunnel> {
    let mut useful_connections = Vec::new();
    for connection in &start_valve.links {
        let connecting_valve = &valves[connection.to_valve_index];

        // Avoid infinite loops
        if path.iter().find(|&&name| name == connecting_valve.name).is_some() {
            continue;
        }

        if connecting_valve.flow_rate > 0 {
            // This direct tunnel is useful
            useful_connections.push(Tunnel { distance: 1, to_valve_index: connection.to_valve_index })
        } else {
            // This connecting valve is useless, so find an indirect tunnel to a useful valve instead
            path.push(start_valve.name.as_str()); // 
            let mut indirect_tunnels = find_paths_to_useful_valves(valves, &connecting_valve, path);
            path.pop();
            indirect_tunnels.iter_mut().for_each(|tunnel| tunnel.distance += 1);
            useful_connections.extend(indirect_tunnels);
        }
    }

    return useful_connections;
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

    // Collapse paths through useless valves
    let collapsed_valves = indexed_valves.iter()
        .filter(|valve| valve.flow_rate > 0 || valve.name == "AA")
        .map(|valve| {
            return Valve {
                name: valve.name.to_string(),
                flow_rate: valve.flow_rate,
                links: find_paths_to_useful_valves(&indexed_valves, &valve, &mut Vec::new()) };
        }).collect_vec();

    return collapsed_valves;

}

fn print_valves(valves: &Vec<Valve>) {
    for valve in valves {
        println!("Valve: {} worth {}", valve.name, valve.flow_rate);
        for connection in &valve.links {
            println!("  {} is a distance of {}", valves[connection.to_valve_index].name, connection.distance );
        }
    }
}

fn find_most_steam<'a>(valves: &'a Vec<Valve>, start_valve_i: usize, time_remaining: i32, open_valves: &mut Vec<usize>, valves_visited_since_last_open: &mut Vec<usize>) -> i32 {
    let start_valve = &valves[start_valve_i];
    let mut max_additional_steam_released = 0;

    if valves_visited_since_last_open.contains(&start_valve_i) {
        return 0; // This route is going around in circles without being productive/opening a valve
    }

    // If we opened this valve
    if ! open_valves.contains(&start_valve_i) && start_valve.flow_rate > 0 {
        let time_to_turn_on_valve = 1;
        let valve_open_time = time_remaining - time_to_turn_on_valve;
        let steam_value = valve_open_time * start_valve.flow_rate;
    
        open_valves.push(start_valve_i);
        for connection in &start_valve.links {
            let time_remaining_at_next_valve = time_remaining - time_to_turn_on_valve - connection.distance;
            if time_remaining_at_next_valve <= 0 {
                // Walking there isn't going to accomplish anything
                continue;
            }

            let additional_steam_released = find_most_steam(
                &valves,
                connection.to_valve_index,
                time_remaining_at_next_valve,
                open_valves,
                &mut Vec::with_capacity(valves.len())
            );
            max_additional_steam_released = max_additional_steam_released.max(steam_value + additional_steam_released);
        }
        open_valves.pop();
    }

    // If we didn't open this valve
    valves_visited_since_last_open.push(start_valve_i);
    if time_remaining > 1 {
        for connection in &start_valve.links {
            let time_remaining_at_next_valve = time_remaining - connection.distance;
            if time_remaining_at_next_valve <= 0 {
                // Walking there isn't going to accomplish anything
                continue;
            }
    
            let additional_steam_released = find_most_steam(
                &valves,
                connection.to_valve_index,
                time_remaining_at_next_valve,
                open_valves,
                valves_visited_since_last_open
            );
            max_additional_steam_released = max_additional_steam_released.max(additional_steam_released);
        }    
    }
    valves_visited_since_last_open.pop();

    return max_additional_steam_released;
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
    
    // Find the highest total steam release by looking at all possible valve visitation routes from the start
    // let start_valve_i = valves.iter().find_position(|valve| valve.name == "AA").unwrap().0;
    // let mut largest_pressure_released = 0;
    // for path in useful_valves.iter().permutations(useful_valves.len()) {
    //     let mut time_remaining = 30;
    //     let mut total_pressure_released = 0;
    //     let mut previous_i = start_valve_i;
    //     for &next_i in path {
    //         let tunnel_to_valve = &tunnels_to_valves[previous_i][next_i];
    //         time_remaining -= tunnel_to_valve.distance + 1; // Include time to turn valve next_i

    //         if time_remaining < 0 {
    //             break; // We can't open any more valves down this path within the time limit
    //         }

    //         // Pre-calculate total presure release by end of time period
    //         total_pressure_released += time_remaining * valves[next_i].flow_rate;

    //         previous_i = next_i;
    //     }

    //     largest_pressure_released = largest_pressure_released.max(total_pressure_released);
    // }

    // Find best way to release steam with exhaustive depth-first search
    // let mut total_steam_released = 0;
    // let mut remaining_time = 30;
    // let mut at_value_i = valves.iter().find_position(|valve| valve.name == "AA").unwrap().0;
    // let mut valves_open = Vec::with_capacity(valves.len());
    // while remaining_time > 0 {
    //     let candidates = find_shortest_distance_to_valves(&valves, at_value_i);
    //     let next_valve = candidates.iter().filter_map(|tunnel| {
    //         let time_to_turn_on_valve = tunnel.distance + 1;
    //         if time_to_turn_on_valve > remaining_time {
    //             // We can't get to this valve and turn it on in time
    //             return None;
    //         }
    //         if valves_open.contains(&tunnel.to_valve_index) {
    //             // This valve is already open, so no point going back
    //             return None;
    //         }

    //         let valve_open_time = remaining_time - time_to_turn_on_valve;
    //         let steam_value = valve_open_time * valves[tunnel.to_valve_index].flow_rate;
    //         return Some((tunnel, steam_value));
    //     }).max_by_key(|(_, value)| *value);

    //     if let Some((tunnel, value)) = next_valve {
    //         println!("Moved to valve {} which takes {} minutes", valves[tunnel.to_valve_index].name, tunnel.distance);
    //         println!("  Valve is being opened at minute {}", tunnel.distance + 1);
    //         total_steam_released += value;
    //         remaining_time -= tunnel.distance + 1; // Includes time to turn on valve
    //         at_value_i = tunnel.to_valve_index;
    //         valves_open.push(at_value_i);

    //     } else {
    //         // No more valves can be opened before time runs out
    //         break;
    //     }
    // }

    // let start_valve_i = valves.iter().find_position(|valve| valve.name == "AA").unwrap().0;
    // let most_steam = find_most_steam(
    //     &valves,
    //     start_valve_i,
    //     30,
    //     &mut Vec::with_capacity(valves.len()),
    //     &mut Vec::with_capacity(valves.len())
    // );

    println!("{}", largest_pressure_released);
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