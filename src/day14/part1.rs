use super::lib::{Position2D, Cave, parse_cave};

fn simulate_sand(cave: &Cave, start: Position2D, floor: usize) -> Option<Position2D> {
    let mut sand = start;
    loop {
        if !cave.is_blocked(&sand.below()) {
            // Sand falls down
            sand = sand.below();
            if sand.y >= floor {
                // Sand falling into the void and will never settle
                return None;
            }
            continue;
        } else if !cave.is_blocked(&sand.left().below()) {
            // Sand moves down-left
            sand = sand.left().below();
        } else if !cave.is_blocked(&sand.right().below()) {
            // Sand moves down-right
            sand = sand.right().below();
        } else {
            // Sand settles here
            return Some(sand);
        }
    }
}

pub fn solve(input: &str) -> i32 {
    let mut cave = parse_cave(input);
    let floor = cave.get_lowest_level();

    // Let the sand fall!
    let mut settled_sands = 0;
    let start = Position2D { x: 500, y: 0 };
    loop {
        if let Some(settle_at) = simulate_sand(&cave, start, floor) {
            // Sand has settled
            cave.mark_blocked(settle_at);
            settled_sands += 1;
            //println!("Settled at {:?}", settle_at);
            //println!("{} units settled", settled_sands);
            //cave.print(Position2D {x: 493, y: 0}, Position2D {x: 504, y: 10});
        } else {
            // Sand is now falling into the void
            return settled_sands;
        }
    }
}

pub mod tests {
    use criterion::Criterion;
    use std::path::Path;

    #[test]
    fn verify_example() {
        assert_eq!(super::solve(super::super::INPUT_EXAMPLE), 24);
    }

    #[test]
    fn verify_solution() {
        assert_eq!(super::solve(super::super::INPUT), 1513);
    }

    pub fn benchmark(c: &mut Criterion) {
        let path = Path::new(file!());
        let day = path.parent().unwrap().to_str().unwrap();
        let solution = path.file_stem().unwrap().to_str().unwrap();
        let id = day.to_owned() + "_" + solution; // dayX_partY_solveZ
        c.bench_function( &id, |b| b.iter(|| super::solve(super::super::INPUT)));
    }
}