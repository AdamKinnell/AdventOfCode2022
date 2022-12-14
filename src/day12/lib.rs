use grid::*;
use itertools::Itertools;

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

pub fn find_traversable_neighbors(map: &Map, position: Position2D) -> Vec<(Position2D, &MapSquare)> {
    let this_square = map.get_square_at(&position).unwrap();

    let adjacent_positions = [
        Position2D {x: position.x    , y: position.y - 1}, // Down
        Position2D {x: position.x    , y: position.y + 1}, // Up
        Position2D {x: position.x - 1, y: position.y    }, // Left
        Position2D {x: position.x + 1, y: position.y    }, // Right
    ];

    return adjacent_positions.iter().filter_map(|neighbor_position| {
        if let Some(square) = map.get_square_at(&neighbor_position) {
            if square.get_height() <= this_square.get_height() + 1 {
                return Some((*neighbor_position, square));
            } else {
                return None;
            }
        } else {
            return None;
        }
    }).collect_vec();
}