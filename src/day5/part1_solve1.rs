use super::solve1_lib;

pub fn solve(input: &str) -> String {
    let (crates_diagram, instructions) = input.split_once("\r\n\r\n").unwrap();
    let crates = &mut solve1_lib::parse_crate_positions(crates_diagram);
    //print_crates(&crates);

    instructions
        .lines()
        .for_each(|instruction| {
            solve1_lib::apply_instruction(instruction, crates);
            //print_crates(&crates);
        });

    return solve1_lib::find_top_of_stacks(&crates);
}

pub mod tests {
    use criterion::Criterion;
    use std::path::Path;

    #[test]
    fn verify_example() {
        assert_eq!(super::solve(super::super::INPUT_EXAMPLE), "CMZ");
    }

    #[test]
    fn verify_solution() {
        assert_eq!(super::solve(super::super::INPUT), "FZCMJCRHZ");
    }

    pub fn benchmark(c: &mut Criterion) {
        let path = Path::new(file!());
        let day = path.parent().unwrap().to_str().unwrap();
        let solution = path.file_stem().unwrap().to_str().unwrap();
        let id = day.to_owned() + "_" + solution; // dayX_partY_solveZ
        c.bench_function( &id, |b| b.iter(|| super::solve(super::super::INPUT)));
    }
}