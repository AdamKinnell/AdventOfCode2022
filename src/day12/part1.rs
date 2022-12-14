use grid::*;
use super::lib::{Map, MapSquare, find_traversable_neighbors};
use std::collections::VecDeque;

pub fn solve(input: &str) -> i32 {
    let map = Map::new_from_string(input);

    // Dijkstra's algorithm
    let mut unvisited_squares = VecDeque::new();
    let mut dijkstra = Grid::init(map.grid.rows(), map.grid.cols(), (-1, None));

    let start = map.find_first(&MapSquare::Start);
    unvisited_squares.push_back(start);
    *dijkstra.get_mut(start.y as usize, start.x as usize).unwrap() = (-1, Some(start));

    // Find shortest path
    loop {
        let position = unvisited_squares.pop_front().unwrap();
        let (distance, prev) = *dijkstra.get(position.y as usize, position.x as usize).unwrap();
        let prev = prev.unwrap();

        if distance != -1 {
            // Already visited
            continue;
        }

        // Update distance of this square from start
        let (prev_distance, _) = *dijkstra.get(prev.y as usize, prev.x as usize).unwrap();
        let (this_distance, _) = dijkstra.get_mut(position.y as usize, position.x as usize).unwrap();
        *this_distance = prev_distance + 1;

        // Queue all neighbors to be visited
        let neighbors = find_traversable_neighbors(&map, position);
        for (neighbor_position, neighbor_square) in neighbors {
            let (_, neighbor_previous) = dijkstra.get_mut(neighbor_position.y as usize, neighbor_position.x as usize).unwrap();
            *neighbor_previous = Some(position);
            unvisited_squares.push_back(neighbor_position);

            if let MapSquare::Goal = neighbor_square {
                // We found the goal and the path to it
                return prev_distance + 2;
            }
        }
    }
}

pub mod tests {
    use criterion::Criterion;
    use std::path::Path;

    #[test]
    fn verify_example() {
        assert_eq!(super::solve(super::super::INPUT_EXAMPLE), 31);
    }

    #[test]
    fn verify_solution() {
        assert_eq!(super::solve(super::super::INPUT), 528);
    }

    pub fn benchmark(c: &mut Criterion) {
        let path = Path::new(file!());
        let day = path.parent().unwrap().to_str().unwrap();
        let solution = path.file_stem().unwrap().to_str().unwrap();
        let id = day.to_owned() + "_" + solution; // dayX_partY_solveZ
        c.bench_function( &id, |b| b.iter(|| super::solve(super::super::INPUT)));
    }
}