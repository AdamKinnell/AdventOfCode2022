use std::{collections::{BinaryHeap}, cmp::Ordering};

use itertools::Itertools;
use regex::Regex;

pub struct Tunnel {
    pub distance: i32,
    pub to_valve_index: usize,
    pub to_valve_flow_rate: i32
}

pub struct Valve {
    pub name: String,
    pub flow_rate: i32,
    pub links: Vec<Tunnel>
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

pub fn find_shortest_distance_to_valves<'a>(valves: &'a Vec<Valve>, start_valve_i: usize) -> Vec<Tunnel> {
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
        .map(|(i, distance)| Tunnel {
            distance: *distance, to_valve_index:
            i,
            to_valve_flow_rate: valves[i].flow_rate
        })
        .collect_vec()
}

pub fn parse_input(input: &str) -> Vec<Valve> {
    let regex = r"^Valve (..) has flow rate=(\d+); tunnels? leads? to valves? (.+)$";
    let re = Regex::new(regex).unwrap();

    // Parse valves
    let valves = input
        .lines()
        .map(|line| {
            let captures = re.captures(line).unwrap();
            let name = captures[1].to_owned();
            let flow_rate: i32 = captures[2].parse().unwrap();
            let connections = captures[3].split(", ").map(str::to_string).collect_vec();
            return (name, flow_rate, connections);
        }).collect_vec();

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
                    .map(|name| Tunnel {
                        distance: 1,
                        to_valve_index: find_valve_index(name.as_str()),
                        to_valve_flow_rate: valves[find_valve_index(name.as_str())].1
                    })
                    .collect_vec()
            })
        .collect_vec();

    return indexed_valves;
}