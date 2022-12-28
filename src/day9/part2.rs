use vector2d::Vector2D;
use std::{collections::HashSet, ops::{Sub, AddAssign}};

trait ToTuple {
    fn to_tuple(&self) -> (i32, i32);
}

impl ToTuple for Vector2D<i32> {
    fn to_tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

fn get_vec_from_direction(direction: &str) -> Vector2D<i32> {
    match direction {
        "U" => Vector2D {x: 0,  y:  1},
        "D" => Vector2D {x: 0,  y: -1},
        "L" => Vector2D {x: -1, y:  0},
        "R" => Vector2D {x:  1, y:  0},
        &_  => unreachable!()
    }
}

struct Snake {
    parts: Vec<Vector2D<i32>>
}

impl Snake {
    fn new(length: usize, start_at: Vector2D<i32>) -> Snake {
        assert!(length >= 1);
        Snake {parts: vec![start_at; length]}
    }

    fn tail_position(&self) -> &Vector2D<i32> {
        return self.parts.last().unwrap();
    }

    fn move_head(&mut self, direction: &str) {
        let dir = get_vec_from_direction(direction);

        // Move head
        self.parts[0].add_assign(dir);

        // Move tail parts one-by-one
        let mut parent_pos = self.parts[0];
        for part in &mut self.parts[1..] {
            let diff = parent_pos.sub(*part);
            if diff.length_squared() > 2 {
                // The parent moved too far away and we have to move this part too
                let movement = Vector2D {
                    x: num::clamp(diff.x, -1, 1),
                    y: num::clamp(diff.y, -1, 1),
                };
                part.add_assign(movement);
                parent_pos = *part;
            } else {
                // If this part doesn't move, later parts won't either.
                break;
            }
        }
    }

}

pub fn solve(input: &str) -> usize {
    let mut snake = Snake::new(10, Vector2D { x: 0, y: 0 });
    let mut tail_visited = HashSet::new();
    tail_visited.insert(snake.tail_position().to_tuple());

    input.lines().for_each(|line| {
        let (dir, dist) = line.split_once(' ').unwrap();

        let distance = dist.parse::<i32>().unwrap();
        for _ in 0..distance {
            snake.move_head(dir);
            tail_visited.insert(snake.tail_position().to_tuple());
        }
    });

    tail_visited.len()
}

pub mod tests {
    use criterion::Criterion;
    use std::path::Path;

    #[test]
    fn verify_example() {
        assert_eq!(super::solve(super::super::INPUT_EXAMPLE_1), 1);
        assert_eq!(super::solve(super::super::INPUT_EXAMPLE_2), 36);
    }

    #[test]
    fn verify_solution() {
        assert_eq!(super::solve(super::super::INPUT), 2367);
    }

    pub fn benchmark(c: &mut Criterion) {
        let path = Path::new(file!());
        let day = path.parent().unwrap().to_str().unwrap();
        let solution = path.file_stem().unwrap().to_str().unwrap();
        let id = day.to_owned() + "_" + solution; // dayX_partY_solveZ
        c.bench_function( &id, |b| b.iter(|| super::solve(super::super::INPUT)));
    }
}