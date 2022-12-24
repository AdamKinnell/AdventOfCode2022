use std::collections::{VecDeque, HashMap};
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
    lowest_filled_row: i32,
    column_heights: [i32; WIDTH],
    formations: VecDeque<[bool; WIDTH]>,
}

impl<const WIDTH: usize> Chamber<WIDTH> {
    fn new() -> Chamber<WIDTH> {
        return Chamber {
            highest_free_row: 0,
            lowest_filled_row: -1,
            column_heights: [0; WIDTH],
            formations: [[false; WIDTH]].into_iter().collect()
        }
    }

    fn get_row(&self, y: i32) -> &[bool; WIDTH] {
        &self.formations[(y - self.lowest_filled_row - 1) as usize]
    }

    fn get_row_mut(&mut self, y: i32) -> &mut [bool; WIDTH] {
        &mut self.formations[(y - self.lowest_filled_row - 1) as usize]
    }

    fn is_formation_at_position(&self, position: &Position2D) -> bool {
        if position.y >= self.highest_free_row {
            return false;
        }

        if position.y <= self.lowest_filled_row {
            return true;
        }

        return self.get_row(position.y)[position.x as usize];
    }

    fn mark_formation_at_position(&mut self, position: &Position2D) {
        if position.y <= self.lowest_filled_row {
            // We assume everything below a certain point is filled anyway
            return;
        }

        // Check if we have enough space on top
        if position.y >= self.highest_free_row as i32 {
            // We must first allocate more room in the chamber
            let rows_to_add = position.y + 1 - self.highest_free_row;
            self.highest_free_row = position.y + 1;
            for _ in 0..rows_to_add {
                self.formations.push_back([false; WIDTH])

            }
        }

        // Mark formation
        self.get_row_mut(position.y)[position.x as usize] = true;

        // Update column heights
        let column_height = &mut self.column_heights[position.x as usize];
         *column_height = *column_height.max(&mut position.y.clone());

        // Check if we can drop the bottom to save memory
        let extra_rows_to_keep = 10; // Adjust as necessary to ensure hashing works for the input
        let new_lowest_filled_row = *self.column_heights.iter().min().unwrap() - extra_rows_to_keep;
        let rows_to_drop = new_lowest_filled_row - self.lowest_filled_row;
        if rows_to_drop > 0 {
            for _ in 0..rows_to_drop {
                self.formations.pop_front();
            }
            self.lowest_filled_row = new_lowest_filled_row;
        }
    }  

    fn rock_fits_at(&self, rock: &Rock, bottom_left_pos: &Position2D) -> bool {
        // Is rock outside the bounds of the chamber?
        if bottom_left_pos.x < 0 { return false; }    // Too far left
        if bottom_left_pos.y < 0 { return false; }    // Too far down 
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
                // Keep simulating after drop
                bottom_left_pos = candidate_pos;
                continue;
            } else {
                // We can't move down any further
                self.stamp_formation(rock, &bottom_left_pos);
                return;
            }

        }
    }

    // fn print(&self) {
    //     for y in (0..self.formations.len()).rev() {
    //         print!("{}: |", y);
    //         for x in self.formations[y] {
    //             if x {
    //                 print!("#")
    //             } else {
    //                 print!(".")
    //             }
    //         }
    //         println!("|")
    //     }
    //     println!("+-------+")
    // }

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

pub fn solve(input: &str, num_rocks: usize) -> usize {
    let gas_jets = parse_gas_jets(input);
    let mut infinite_gas_jets = gas_jets.into_iter().cycle();
    let mut chamber: Chamber<7> = Chamber::new();

    // Initialize cache
    let mut height_after_rock = Vec::new();
    let mut seen_states = HashMap::new();

    for (i, rock) in get_infinite_rocks().take(num_rocks).enumerate() {
        chamber.simulate_rock(&rock, &mut infinite_gas_jets);

        height_after_rock.push(chamber.highest_free_row as usize);
        let seen = seen_states.insert(chamber.formations.clone(), i);

        if let Some(cycle_start_i) = seen {
            // We're starting to repeat and we found the cycle point
            
            let cycle_period = i - cycle_start_i;
            println!("Input cycles starting from rock {} with period {}", cycle_start_i, cycle_period);
            
            let remaining_rocks = num_rocks - (i + 1);
            let current_height = height_after_rock[i];

            let remaining_full_cycles = remaining_rocks / cycle_period;
            let height_increase_per_full_cycle = height_after_rock[i] - height_after_rock[cycle_start_i];
            let height_from_remaining_full_cycles = remaining_full_cycles * height_increase_per_full_cycle;

            let rocks_in_remainder_cycle = remaining_rocks % cycle_period;
            let height_from_remainder_cycle = height_after_rock[cycle_start_i + rocks_in_remainder_cycle] - height_after_rock[cycle_start_i];
            
            return current_height + height_from_remaining_full_cycles + height_from_remainder_cycle;
        }
    }

    // No cycles found in input
    return chamber.highest_free_row as usize;
}

pub mod tests {
    use criterion::Criterion;
    use std::path::Path;

    #[test]
    fn verify_example_2022() {
        assert_eq!(super::solve(super::super::INPUT_EXAMPLE, 2022), 3068);
    }

    #[test]
    fn verify_example() {
        assert_eq!(super::solve(super::super::INPUT_EXAMPLE, 1_000_000_000_000), 1514285714288);
    }

    #[test]
    fn verify_solution_2022() {
        assert_eq!(super::solve(super::super::INPUT, 2022), 3191);
    }

    #[test]
    fn verify_solution() {
        assert_eq!(super::solve(super::super::INPUT, 1_000_000_000_000), 1_572_093_023_267);
    }

    pub fn benchmark(c: &mut Criterion) {
        let path = Path::new(file!());
        let day = path.parent().unwrap().to_str().unwrap();
        let solution = path.file_stem().unwrap().to_str().unwrap();
        let id = day.to_owned() + "_" + solution; // dayX_partY_solveZ
        c.bench_function( &id, |b| b.iter(|| super::solve(super::super::INPUT, 1_000_000_000_000)));
    }
}