use super::lib::{Map, MapSquare, dijkstra_find_shortest_path_length};

pub fn solve(input: &str) -> i32 {
    let map = Map::new_from_string(input);
    let can_move_to = |to: &MapSquare, from: &MapSquare| to.get_height() <= from.get_height() + 1;
    let start = map.find_first(&MapSquare::Goal);
    let goal = MapSquare::Terrain(0);
    return dijkstra_find_shortest_path_length(&map, start, &goal, &can_move_to).unwrap();
}

pub mod tests {
    use criterion::Criterion;
    use std::path::Path;

    #[test]
    fn verify_example() {
        assert_eq!(super::solve(super::super::INPUT_EXAMPLE), 522);
    }

    #[test]
    fn verify_solution() {
        assert_eq!(super::solve(super::super::INPUT), -1);
    }

    pub fn benchmark(c: &mut Criterion) {
        let path = Path::new(file!());
        let day = path.parent().unwrap().to_str().unwrap();
        let solution = path.file_stem().unwrap().to_str().unwrap();
        let id = day.to_owned() + "_" + solution; // dayX_partY_solveZ
        c.bench_function( &id, |b| b.iter(|| super::solve(super::super::INPUT)));
    }
}