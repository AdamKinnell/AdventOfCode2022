use super::lib::{Position2D, Cave, parse_cave};

struct SandSimulator<'a> {
    cache: Vec<Position2D>,
    sands_settled: i32,
    floor: usize,
    cave: &'a mut Cave,
}

impl SandSimulator<'_> {
    fn new(cave: &mut Cave, floor: usize) -> SandSimulator {
        SandSimulator {
            cache: Vec::new(),
            sands_settled: 0,
            floor,
            cave,
        }
    }

    fn drop_sand(&mut self, from: Position2D) -> Option<Position2D> {

        let is_blocked = |position: &Position2D| position.y > self.floor || self.cave.is_blocked(position);

        let mut sand;
        if self.cache.first().is_some() && self.cache.first().unwrap() == &from {
            // Existing cache exists and is valid for this starting point
            sand = *self.cache.last().unwrap();
        } else {
            // Cache is empty or not valid
            self.cache.clear();
            self.cache.push(from);
            sand = from;
        }

        loop {
            if !is_blocked(&sand.below()) {
                // Sand falls down
                sand = sand.below();
                self.cache.push(sand);
            } else if !is_blocked(&sand.left().below()) {
                // Sand moves down-left
                sand = sand.left().below();
                self.cache.push(sand);
            } else if !is_blocked(&sand.right().below()) {
                // Sand moves down-right
                sand = sand.right().below();
                self.cache.push(sand);
            } else if sand == from && is_blocked(&sand) {
                // Spout is blocked
                return None;
            } else {
                // Sand settles here
                self.sands_settled += 1;
                self.cave.mark_blocked(sand);
                self.cache.pop();
                return Some(sand);
            }
        }
    }
}

pub fn solve(input: &str) -> i32 {
    let mut cave = parse_cave(input);

    // Let the sand fall!
    let start = Position2D { x: 500, y: 0 };
    let floor = cave.get_lowest_level();
    let mut simulator = SandSimulator::new(&mut cave, floor);

    while simulator.drop_sand(start).is_some() {}

    simulator.sands_settled
}

pub mod tests {
    use criterion::Criterion;
    use std::path::Path;

    #[test]
    fn verify_example() {
        assert_eq!(super::solve(super::super::INPUT_EXAMPLE), 93);
    }

    #[test]
    fn verify_solution() {
        assert_eq!(super::solve(super::super::INPUT), 22646);
    }

    pub fn benchmark(c: &mut Criterion) {
        let path = Path::new(file!());
        let day = path.parent().unwrap().to_str().unwrap();
        let solution = path.file_stem().unwrap().to_str().unwrap();
        let id = day.to_owned() + "_" + solution; // dayX_partY_solveZ
        c.bench_function( &id, |b| b.iter(|| super::solve(super::super::INPUT)));
    }
}