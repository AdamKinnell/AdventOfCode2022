use super::lib::Command;

pub fn solve(input: &str) -> usize {
    let commands = input.split("$ ")
        .filter(|s| ! s.is_empty())
        .map(Command::parse)
        .chain(std::iter::repeat(Command::ReturnToParent)); // Make sure we go back up towards the root directory

    let mut sum_of_dirs_less_than_100000 = 0usize;
    super::lib::traverse_directory(commands, | dir_size | {
        if dir_size <= 100000 {
            sum_of_dirs_less_than_100000 += dir_size;
        }    
    });

    sum_of_dirs_less_than_100000
}

pub mod tests {
    use criterion::Criterion;
    use std::path::Path;

    #[test]
    fn verify_example() {
        assert_eq!(super::solve(super::super::INPUT_EXAMPLE), 95437);
    }

    #[test]
    fn verify_solution() {
        assert_eq!(super::solve(super::super::INPUT), 1743217);
    }

    pub fn benchmark(c: &mut Criterion) {
        let path = Path::new(file!());
        let day = path.parent().unwrap().to_str().unwrap();
        let solution = path.file_stem().unwrap().to_str().unwrap();
        let id = day.to_owned() + "_" + solution; // dayX_partY_solveZ
        c.bench_function( &id, |b| b.iter(|| super::solve(super::super::INPUT)));
    }
}