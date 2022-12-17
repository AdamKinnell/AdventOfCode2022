use std::collections::{HashSet};
use itertools::Itertools;

#[derive(Clone, Eq, PartialEq, Hash, Debug, Copy)]
pub struct Position2D {
    pub x: usize,
    pub y: usize
}

impl Position2D {
    pub fn below(&self) -> Position2D {
        return Position2D {x: self.x, y: self.y + 1};
    }

    pub fn left(&self) -> Position2D {
        return Position2D {x: self.x - 1, y: self.y};
    }

    pub fn right(&self) -> Position2D {
        return Position2D {x: self.x + 1, y: self.y};
    }
}

pub struct Cave {
    blocked: HashSet::<Position2D>
}

impl Cave {
    pub fn get_lowest_level(&self) -> usize {
        return self.blocked.iter().map(|position| position.y).max().unwrap()
    }

    pub fn print(&self, min: Position2D, max: Position2D) {
        for y in (min.y)..=(max.y) {
            for x in (min.x)..=(max.x) {
                if self.blocked.contains(&Position2D { x,y }) {
                    print!("X");
                } else {
                    print!(".")
                }
            }
            println!("");
        }
    }

    pub fn is_blocked(&self, position: &Position2D) -> bool {
        return self.blocked.contains(&position);
    }

    pub fn mark_blocked(&mut self, position: Position2D) {
        self.blocked.insert(position);
    }
}

fn draw_rock_lines(from: Position2D, to: Position2D, cave: &mut Cave) {
    // Note: Lines are only ever vertical or horizontal so one of these loops only has a single iteration
    for x in from.x.min(to.x)..=from.x.max(to.x) {
        for y in from.y.min(to.y)..=from.y.max(to.y) {
            cave.blocked.insert(Position2D { x, y });
        }
    }
}

fn parse_rock_formation(input: &str, cave: &mut Cave) {
    input
        .split(" -> ")
        .map(|coord| {
            let (x, y) = coord.split_once(",").unwrap();
            return Position2D {x: x.parse().unwrap(), y: y.parse().unwrap()};
        })
        .tuple_windows()
        .for_each(|(from, to)| {
            draw_rock_lines(from, to, cave)
        });
}

pub fn parse_cave(input: &str) -> Cave {
    let mut cave = Cave { blocked: HashSet::new() };
    input.lines().for_each(|r| parse_rock_formation(r, &mut cave));
    return cave;
}