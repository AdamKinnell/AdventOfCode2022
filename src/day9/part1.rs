use vector2d::Vector2D;
use std::{collections::HashSet, ops::{Add, Sub}};

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

pub fn solve(input: &str) -> usize {
    let mut head_at = Vector2D {x: 0, y:0};
    let mut tail_at = Vector2D {x: 0, y:0};
    
    let mut tail_visited = HashSet::new();
    tail_visited.insert(tail_at.to_tuple());

    input.lines().for_each(|line| {
        let (dir, dist) = line.split_once(" ").unwrap();
        let direction = get_vec_from_direction(dir);
        let distance = dist.parse::<i32>().unwrap();
        
        for _ in 0..distance {
            let head_before_move = head_at;
            head_at = head_at.add(direction);
            if head_at.sub(tail_at).length_squared() > 2 {
                // Head is no longer in one of the 9 spaces on or around the tail
                tail_at = head_before_move;
                tail_visited.insert(tail_at.to_tuple());
            }
        }
    });

    return tail_visited.len();
}

pub mod tests {
    use criterion::Criterion;
    use std::path::Path;

    #[test]
    fn verify_example() {
        assert_eq!(super::solve(super::super::INPUT_EXAMPLE_1), 13);
    }

    #[test]
    fn verify_solution() {
        assert_eq!(super::solve(super::super::INPUT), 5883);
    }

    pub fn benchmark(c: &mut Criterion) {
        let path = Path::new(file!());
        let day = path.parent().unwrap().to_str().unwrap();
        let solution = path.file_stem().unwrap().to_str().unwrap();
        let id = day.to_owned() + "_" + solution; // dayX_partY_solveZ
        c.bench_function( &id, |b| b.iter(|| super::solve(super::super::INPUT)));
    }
}