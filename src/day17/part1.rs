use grid::Grid;
use itertools::Itertools;

#[derive(Debug)]
struct Position2D {
    x: i32,
    y: i32
}

impl Position2D {
    fn move_dir(&self, direction: Direction) -> Position2D {
        match direction {
            Direction::Left => Position2D { x: self.x - 1, y: self.y },
            Direction::Right => Position2D { x: self.x + 1, y: self.y },
            Direction::Down => Position2D { x: self.x, y: self.y - 1 },
        }
    }
}

#[derive(Clone)]
struct Rock {
    shape: Grid<bool>
}

impl Rock {
    fn get_offset_shape_coordinates<'a>(&'a self, bottom_left_pos: &'a Position2D) -> impl Iterator<Item=Position2D> + 'a {
        let row_indices = 0..self.shape.rows();
        let column_indices = 0..self.shape.cols();

        return row_indices.cartesian_product(column_indices)
            .filter(|(y, x)| self.shape[*y][*x])
            .map(|(y, x)| {
                // Convert from rock to chamber coordinate
                let chamber_y = bottom_left_pos.y + (self.shape.rows() - 1 - y) as i32;
                let chamber_x = bottom_left_pos.x + x as i32;
                return Position2D {x: chamber_x, y: chamber_y};
            });
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
    Down
}

struct Chamber<const WIDTH: usize> {
    highest_free_row: i32,
    formations: Vec<[bool; WIDTH]>,
}

impl<const WIDTH: usize> Chamber<WIDTH> {
    fn new() -> Chamber<WIDTH> {
        return Chamber {
            highest_free_row: 0,
            formations: Vec::new()
        }
    }

    fn is_formation_at_position(&self, position: &Position2D) -> bool {
        if position.y >= self.highest_free_row {
            return false;
        }

        return self.formations[position.y as usize][position.x as usize];
    }

    fn mark_formation_at_position(&mut self, position: &Position2D) {
        if position.y >= self.formations.len() as i32 {
            // Allocate more room in the chamber
            self.highest_free_row = position.y + 1;
            while self.formations.len() < (position.y + 1) as usize {
                self.formations.push([false; WIDTH])
            }
        }
        self.formations[position.y as usize][position.x as usize] = true;
    }  

    fn rock_fits_at(&self, rock: &Rock, bottom_left_pos: &Position2D) -> bool {
        // Is rock outside the bounds of the chamber?
        if bottom_left_pos.x < 0 { return false; }                  // Too far left
        if bottom_left_pos.y < 0 { return false; }                  // Too far down 
        let right_pos =bottom_left_pos.x + (rock.shape.cols() as i32 - 1);
        if right_pos >= WIDTH as i32 { return false; } // Too far right

        // Is rock above all existing formations?
        if bottom_left_pos.y >= self.highest_free_row {
            return true
        }

        // Is rock obstructed by an existing settled formation?
        for chamber_pos in rock.get_offset_shape_coordinates(bottom_left_pos) {
            if self.is_formation_at_position(&chamber_pos) {
                // This part of the rock overlaps with an existing formation
                return false;
            }
        }

        return true;
    }

    fn stamp_formation(&mut self, rock: &Rock, bottom_left_pos: &Position2D) {
        for chamber_pos in rock.get_offset_shape_coordinates(bottom_left_pos) {
            self.mark_formation_at_position(&chamber_pos);
        }
    }

    fn simulate_rock(&mut self, rock: &Rock, gas_jets: &mut impl Iterator<Item = Direction>) {
        let mut bottom_left_pos = Position2D {x: 2, y: self.highest_free_row + 3};
        loop {
            // Push with gas (if it can move in that direction)
            let gas_dir = gas_jets.next().unwrap();
            let candidate_pos = bottom_left_pos.move_dir(gas_dir);

            if self.rock_fits_at(rock, &candidate_pos) {
                bottom_left_pos = candidate_pos;
            }

            // Attempt to drop the rock
            let candidate_pos = bottom_left_pos.move_dir(Direction::Down);
            if self.rock_fits_at(rock, &candidate_pos) {
                // Keep simulating
                bottom_left_pos = candidate_pos;
                continue;
            } else {
                // We can't move down any further
                self.stamp_formation(rock, &bottom_left_pos);
                return;
            }

        }
    }
}

fn get_infinite_rocks() -> impl Iterator<Item=Rock> {
    let shapes = [
        Grid::<bool>::from_vec([true, true, true, true].to_vec(), 4), // Line
        Grid::<bool>::from_vec([false, true, false,
                                 true, true, true,
                                false, true, false].to_vec(), 3), // Plus
        Grid::<bool>::from_vec([false, false, true,
                                false, false, true,
                                 true,  true, true].to_vec(), 3), // Corner
        Grid::<bool>::from_vec([true, true, true, true].to_vec(), 1), // Pipe
        Grid::<bool>::from_vec([true, true, true, true].to_vec(), 2), // Quad
    ];

    return shapes.into_iter().map(|shape| Rock {shape}).cycle();
}

fn parse_gas_jets(input: &str) -> Vec<Direction> {
    return input.chars().map(|c| match c {
        '<' => Direction::Left,
        '>' => Direction::Right,
        _ => unreachable!()
    }).collect_vec();
}

pub fn solve(input: &str) -> i32 {
    let mut gas_jets = parse_gas_jets(input).into_iter().cycle();
    let mut chamber: Chamber<7> = Chamber::new();
    for rock in get_infinite_rocks().take(2022) {
        chamber.simulate_rock(&rock, &mut gas_jets);
    }

    return chamber.highest_free_row;
}

pub mod tests {
    use criterion::Criterion;
    use std::path::Path;

    #[test]
    fn verify_example() {
        assert_eq!(super::solve(super::super::INPUT_EXAMPLE), 3068);
    }

    #[test]
    fn verify_solution() {
        assert_eq!(super::solve(super::super::INPUT), 3191);
    }

    pub fn benchmark(c: &mut Criterion) {
        let path = Path::new(file!());
        let day = path.parent().unwrap().to_str().unwrap();
        let solution = path.file_stem().unwrap().to_str().unwrap();
        let id = day.to_owned() + "_" + solution; // dayX_partY_solveZ
        c.bench_function( &id, |b| b.iter(|| super::solve(super::super::INPUT)));
    }
}