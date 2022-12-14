use grid::*;
use itertools::Itertools;
use std::collections::VecDeque;

#[derive(Eq, PartialEq)]
pub enum MapSquare {
    Start,
    Goal,
    Terrain(u8)
}

impl MapSquare {
    pub fn from_char(char: char) -> Option<MapSquare> {
        match char {
            'a'..='z' => Some(MapSquare::Terrain(char as u8 - 'a' as u8)),
            'S' => Some(MapSquare::Start),
            'E' => Some(MapSquare::Goal),
              _ => None
        }
    }

    pub fn get_height(&self) -> u8 {
        match self {
            MapSquare::Start => 0,
            MapSquare::Goal => 25,
            MapSquare::Terrain(height) => *height,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Position2D {
    pub x: i32,
    pub y: i32
}

pub struct Map {
    pub grid: Grid<MapSquare>
}

impl Map {
    pub fn new_from_string(input: &str) -> Map {
        let width = input.find("\r\n").unwrap();
        let squares = input
            .chars()
            .filter_map(MapSquare::from_char)
            .collect_vec();
        return Map {
            grid: Grid::from_vec(squares, width)
        };
    }

    pub fn get_square_at(&self, position: &Position2D) -> Option<&MapSquare> {
        if position.x < 0 || position.y < 0 {
            return None;
        }
        return self.grid.get(position.y as usize, position.x as usize);
    }

    pub fn find_first(&self, square_type: &MapSquare) -> Position2D {
        for y in 0..self.grid.rows() {
            for x in 0..self.grid.cols() {
                if self.grid.get(y, x).unwrap() == square_type {
                    return Position2D {x: x as i32, y: y as i32};

                }
            }
        }
        unreachable!();
    }
}

pub fn find_traversable_neighbors<'a>(map: &'a Map, position: Position2D, can_move_to: &impl Fn(&MapSquare, &MapSquare) -> bool) -> Vec<(Position2D, &'a MapSquare)> {
    let this_square = map.get_square_at(&position).unwrap();

    let adjacent_positions = [
        Position2D {x: position.x    , y: position.y - 1}, // Down
        Position2D {x: position.x    , y: position.y + 1}, // Up
        Position2D {x: position.x - 1, y: position.y    }, // Left
        Position2D {x: position.x + 1, y: position.y    }, // Right
    ];

    return adjacent_positions.iter().filter_map(|neighbor_position| {
        if let Some(square) = map.get_square_at(&neighbor_position) {
            if can_move_to(&this_square, &square) {
                return Some((*neighbor_position, square));
            } else {
                return None;
            }
        } else {
            return None;
        }
    }).collect_vec();
}

// Dijkstra's algorithm
pub fn dijkstra_find_shortest_path_length(map: &Map, start: Position2D, goal: &MapSquare, can_move_to: &impl Fn(&MapSquare, &MapSquare) -> bool) -> Option<i32> {
        let mut unvisited_squares = VecDeque::new();
        let mut algorithm_state = Grid::init(map.grid.rows(), map.grid.cols(), (-1, None));
    
        unvisited_squares.push_back(start);
        *algorithm_state.get_mut(start.y as usize, start.x as usize).unwrap() = (-1, Some(start));
    
        // Find shortest path
        while !unvisited_squares.is_empty() {
            let position = unvisited_squares.pop_front().unwrap();
            let (distance, prev) = *algorithm_state.get(position.y as usize, position.x as usize).unwrap();
            let prev = prev.unwrap();
    
            if distance != -1 {
                // Already visited
                continue;
            }
    
            // Update distance of this square from start
            let (prev_distance, _) = *algorithm_state.get(prev.y as usize, prev.x as usize).unwrap();
            let (this_distance, _) = algorithm_state.get_mut(position.y as usize, position.x as usize).unwrap();
            *this_distance = prev_distance + 1;
    
            // Queue all neighbors to be visited
            let neighbors = find_traversable_neighbors(&map, position, can_move_to);
            for (neighbor_position, neighbor_square) in neighbors {
                let (_, neighbor_previous) = algorithm_state.get_mut(neighbor_position.y as usize, neighbor_position.x as usize).unwrap();
                *neighbor_previous = Some(position);
                unvisited_squares.push_back(neighbor_position);
    
                if neighbor_square == goal {
                    // We found the goal and the path to it
                    return Some(prev_distance + 2); // distance of previous + this tile (1) + neighbor/goal tile (1)
                }
            }
        }

    //
    return None;
}