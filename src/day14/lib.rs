use grid::Grid;
use itertools::Itertools;

#[derive(Clone, Eq, PartialEq, Hash, Debug, Copy)]
pub struct Position2D {
    pub x: usize,
    pub y: usize
}

impl Position2D {
    pub fn below(&self) -> Position2D {
        Position2D {x: self.x, y: self.y + 1}
    }

    pub fn left(&self) -> Position2D {
        Position2D {x: self.x - 1, y: self.y}
    }

    pub fn right(&self) -> Position2D {
        Position2D {x: self.x + 1, y: self.y}
    }
}

pub struct Cave {
    blocked: Grid<bool>,
    offset_x: usize,
    offset_y: usize,
}

impl Cave {

    pub fn new(section_min: Position2D, section_max: Position2D) -> Cave {
        let blocked = Grid::init(
             section_max.y - section_min.y + 1,
             section_max.x - section_min.x + 1,
             false);
        Cave { blocked, offset_x: section_min.x, offset_y: section_min.y }
    }

    pub fn get_lowest_level(&self) -> usize {
        self.offset_y + self.blocked.rows() - 1
    }

    pub fn print(&self) {
        for y in 0..self.blocked.rows() {
            for x in 0..self.blocked.cols() {
                if self.is_blocked(&Position2D { x,y }) {
                    print!("X");
                } else {
                    print!(".")
                }
            }
            println!();
        }
    }

    pub fn is_blocked(&self, position: &Position2D) -> bool {
        return *self.blocked.get(
            position.y - self.offset_y,
            position.x - self.offset_x).unwrap()
    }

    pub fn mark_blocked(&mut self, position: Position2D) {
        *self.blocked.get_mut(
            position.y - self.offset_y,
            position.x - self.offset_x).unwrap() = true;
    }
}

fn draw_rock_lines(from: Position2D, to: Position2D, cave: &mut Cave) {
    // Note: Lines are only ever vertical or horizontal so one of these loops only has a single iteration
    for x in from.x.min(to.x)..=from.x.max(to.x) {
        for y in from.y.min(to.y)..=from.y.max(to.y) {
            cave.mark_blocked(Position2D {x,y})
        }
    }
}

fn parse_rock_formation(input: &str, cave: &mut Cave) {
    input
        .split(" -> ")
        .map(|coord| {
            let (x, y) = coord.split_once(',').unwrap();
            Position2D {x: x.parse().unwrap(), y: y.parse().unwrap()}
        })
        .tuple_windows()
        .for_each(|(from, to)| {
            draw_rock_lines(from, to, cave)
        });
}

fn find_rock_formation_bounds(input: &str) -> (Position2D, Position2D) {
    let mut min = Position2D {x: usize::MAX, y: usize::MAX};
    let mut max = Position2D {x: 0, y: 0};

    input.lines()
        .flat_map(|line| line.split(" -> "))
        .map(|coord| {
            let (x, y) = coord.split_once(',').unwrap();
            Position2D {x: x.parse().unwrap(), y: y.parse().unwrap()}
        })
        .for_each(|position| {
            min.x = position.x.min(min.x);
            min.y = position.y.min(min.y);
            max.x = position.x.max(max.x);
            max.y = position.y.max(max.y);
        });

    (min, max)
}

pub fn parse_cave(input: &str) -> Cave {
    let (mut min, mut max) = find_rock_formation_bounds(input);
    min.y = 0; // Ensure we can simulate all the way to the roof
    max.y += 1; // Ensure we have a buffer below to allow the infinite floor in Part 2
    max.x = 500 + max.y;
    min.x = 500 - max.y;

    let mut cave = Cave::new(min, max);
    input.lines().for_each(|r| parse_rock_formation(r, &mut cave));
    cave
}