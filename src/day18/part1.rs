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
        Grid3D { grid: vec![default; total_size as usize], x_size, y_size, z_size }
    }

    fn index_3d(&self, pos: Position3D) -> usize {
        (pos.z * (self.x_size * self.y_size) +
               pos.y * self.x_size +
               pos.x) as usize
    }

    fn is_in_bounds(&self, pos: &Position3D) -> bool {
        pos.x >= 0 && pos.x < self.x_size &&
               pos.y >= 0 && pos.y < self.y_size &&
               pos.z >= 0 && pos.z < self.z_size
    }

    fn get(&self, pos: Position3D) -> Option<T> {
        if self.is_in_bounds(&pos) {
            Some(self.grid[self.index_3d(pos)])
        } else {
            None
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
        Position3D {
            x: components.next().unwrap().parse().unwrap(),
            y: components.next().unwrap().parse().unwrap(),
            z: components.next().unwrap().parse().unwrap()
        }
    })
    .collect_vec();
}

pub fn solve(input: &str) -> i32 {
    let cubes = parse_positions(input);

    let x_max = cubes.iter().max_by_key(|c| c.x).unwrap().x;
    let y_max = cubes.iter().max_by_key(|c| c.y).unwrap().y;
    let z_max = cubes.iter().max_by_key(|c| c.z).unwrap().z;
    let mut grid = Grid3D::new(x_max + 1, y_max + 1, z_max + 1, false);
    for cube in cubes {
        grid.set(cube, true);
    }

    //println!("Status after {},{},{}", cube.x, cube.y, cube.z);
    // for z in 0..grid.z_size {
    //     println!("\nZ = {}", z);
    //     for y in 0..grid.y_size {
    //         for x in 0..grid.x_size {
    //             let pos = Position3D { x, y, z };
    //             if grid.get(pos).unwrap() {
    //                 print!("#")
    //             } else {
    //                 print!(".")
    //             }
    //         }
    //         println!()
    //     }
    // }

    let mut surface_area = 0;
    for z in 0..grid.z_size {
        for y in 0..grid.y_size {
            for x in 0..grid.x_size {

                // Make sure there is a cube here
                if !grid.get(Position3D {x,y,z}).unwrap() {
                    continue;
                }

                // Check neighbors on all 6 sides of the cube
                if !grid.get(Position3D {x: x + 1, y, z }).unwrap_or(false) {surface_area += 1}
                if !grid.get(Position3D {x: x - 1, y, z }).unwrap_or(false) {surface_area += 1}
                if !grid.get(Position3D {x, y: y + 1, z }).unwrap_or(false) {surface_area += 1}
                if !grid.get(Position3D {x, y: y - 1, z }).unwrap_or(false) {surface_area += 1}
                if !grid.get(Position3D {x, y, z: z + 1 }).unwrap_or(false) {surface_area += 1}
                if !grid.get(Position3D {x, y, z: z - 1 }).unwrap_or(false) {surface_area += 1}
            }
        }
    }

    surface_area
}

pub mod tests {
    use criterion::Criterion;
    use std::path::Path;

    #[test]
    fn verify_example() {
        assert_eq!(super::solve(super::super::INPUT_EXAMPLE), 64);
    }

    #[test]
    fn verify_example_2() {
        assert_eq!(super::solve(super::super::INPUT_EXAMPLE_2), 60); // 3x3 hollow cube
    }

    #[test]
    fn verify_solution() {
        assert_eq!(super::solve(super::super::INPUT), 4460);
    }

    pub fn benchmark(c: &mut Criterion) {
        let path = Path::new(file!());
        let day = path.parent().unwrap().to_str().unwrap();
        let solution = path.file_stem().unwrap().to_str().unwrap();
        let id = day.to_owned() + "_" + solution; // dayX_partY_solveZ
        c.bench_function( &id, |b| b.iter(|| super::solve(super::super::INPUT)));
    }
}