use super::lib::get_cycle_iterator;

pub fn solve(input: &str) -> i32 {
    let cycles = get_cycle_iterator(input);
    let mut total = 0;
    for (cycle, register) in cycles.enumerate() {
        if [20, 60, 100, 140, 180, 220].contains(&cycle) {
            total += register * cycle as i32;
        }
        //println!("{}: {}", cycle, register);
    }

    return total;
}

pub mod tests {
    use criterion::Criterion;
    use std::path::Path;

    #[test]
    fn verify_example() {
        assert_eq!(super::solve(super::super::INPUT_EXAMPLE), 13140);
    }

    #[test]
    fn verify_solution() {
        assert_eq!(super::solve(super::super::INPUT), 15880);
    }

    pub fn benchmark(c: &mut Criterion) {
        let path = Path::new(file!());
        let day = path.parent().unwrap().to_str().unwrap();
        let solution = path.file_stem().unwrap().to_str().unwrap();
        let id = day.to_owned() + "_" + solution; // dayX_partY_solveZ
        c.bench_function( &id, |b| b.iter(|| super::solve(super::super::INPUT)));
    }
}