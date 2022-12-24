use std::collections::VecDeque;

use itertools::Itertools;

#[derive(Copy, Clone)]
struct Position3D {
    x: i32,
    y: i32,
    z: i32
}

struct Grid3D<T> {
    grid: Vec<T>,
    x_size: i32,
    y_size: i32,
    z_size: i32
}

impl<T: Copy> Grid3D<T> {
    fn new(x_size: i32, y_size: i32, z_size: i32, default: T) -> Grid3D<T> {
        let total_size = x_size * y_size * z_size;
        return Grid3D { grid: vec![default; total_size as usize], x_size, y_size, z_size };
    }

    fn index_3d(&self, pos: Position3D) -> usize {
        return (pos.z * (self.x_size * self.y_size) +
               pos.y * self.x_size +
               pos.x) as usize;
    }

    fn is_in_bounds(&self, pos: &Position3D) -> bool {
        return pos.x >= 0 && pos.x < self.x_size &&
               pos.y >= 0 && pos.y < self.y_size &&
               pos.z >= 0 && pos.z < self.z_size
    }

    fn get(&self, pos: Position3D) -> Option<T> {
        if self.is_in_bounds(&pos) {
            return Some(self.grid[self.index_3d(pos)]);
        } else {
            return None;
        }
    }

    fn set(&mut self, pos: Position3D, value: T) {
        let index = self.index_3d(pos);
        self.grid[index] = value;
    }
}

fn parse_positions(input: &str) -> Vec<Position3D> {
    return input
    .lines()
    .map(|coord| {
        let mut components = coord.split(',');
        return Position3D {
            x: components.next().unwrap().parse().unwrap(),
            y: components.next().unwrap().parse().unwrap(),
            z: components.next().unwrap().parse().unwrap()
        };
    })
    .collect_vec();
}

pub fn solve(input: &str) -> i32 {
    let cubes = parse_positions(input);

    // Load the given coordinates into a 3d array.
    // Input is assumed to have all coordinates >= 0
    // We create a margin of 1 in all dimensions to allow flood-fill to see faces on the border.
    let x_max = cubes.iter().max_by_key(|c| c.x).unwrap().x;
    let y_max = cubes.iter().max_by_key(|c| c.y).unwrap().y;
    let z_max = cubes.iter().max_by_key(|c| c.z).unwrap().z;
    let mut grid = Grid3D::new(x_max + 3, y_max + 3, z_max + 3, false);
    for cube in cubes {
        grid.set(Position3D { x: cube.x + 1, y: cube.y + 1, z: cube.z + 1 }, true);
    }

    // Flood fill from outside to count exterior surfaces
    let mut surface_area = 0;
    let mut queue = VecDeque::new();
    let mut checked_grid = Grid3D::new(grid.x_size, grid.y_size, grid.z_size, false);
    queue.push_back(Position3D { x: 0, y:0, z: 0 }); // Start at a point in the margin

    while !queue.is_empty() {
        let this = queue.pop_front().unwrap();

        if checked_grid.get(this) == Some(true) {
            // We already checked this coordinate
            continue;
        }

        if grid.get(this) == Some(true) {
            // We should never add an occupied square to the queue
            unreachable!()
        }

        let neighbors = [
            Position3D {x: this.x + 1, y: this.y,     z: this.z },
            Position3D {x: this.x - 1, y: this.y,     z: this.z },
            Position3D {x: this.x,     y: this.y + 1, z: this.z },
            Position3D {x: this.x,     y: this.y - 1, z: this.z },
            Position3D {x: this.x,     y: this.y,     z: this.z + 1 },
            Position3D {x: this.x,     y: this.y,     z: this.z - 1 },
        ];

        for neighbor in neighbors {
            if let Some(is_occupied) = grid.get(neighbor) {
                if is_occupied {
                    // Found an exterior face
                    surface_area += 1;
                } else {
                    // Found another position to explore
                    queue.push_back(neighbor);
                }
            } else {
                // Neighbor is out-of bounds and we won't check it
                continue;
            }
        }

        // We're done with this position
        checked_grid.set(this, true);
    }

    // for z in 0..grid.z_size {
    //     println!("\nZ = {}", z);
    //     for y in 0..grid.y_size {
    //         for x in 0..grid.x_size {
    //             let pos = Position3D { x, y, z };
    //             if checked_grid.get(pos).unwrap() {
    //                 print!("c")
    //             } else {
    //                 if grid.get(pos).unwrap() {
    //                     print!("#")
    //                 } else {
    //                     print!(".")
    //                 }
    //             }
    //         }
    //         println!()
    //     }
    // }

    return surface_area;
}

pub mod tests {
    use criterion::Criterion;
    use std::path::Path;

    #[test]
    fn verify_example() {
        assert_eq!(super::solve(super::super::INPUT_EXAMPLE), 58);
    }

    #[test]
    fn verify_example_2() {
        assert_eq!(super::solve(super::super::INPUT_EXAMPLE_2), 54); // Hollow 3x3 cube
    }

    #[test]
    fn verify_solution() {
        assert_eq!(super::solve(super::super::INPUT), 2064);
    }

    pub fn benchmark(c: &mut Criterion) {
        let path = Path::new(file!());
        let day = path.parent().unwrap().to_str().unwrap();
        let solution = path.file_stem().unwrap().to_str().unwrap();
        let id = day.to_owned() + "_" + solution; // dayX_partY_solveZ
        c.bench_function( &id, |b| b.iter(|| super::solve(super::super::INPUT)));
    }
}