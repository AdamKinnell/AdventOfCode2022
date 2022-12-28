use super::lib::Command;

pub fn solve(input: &str) -> usize {
    let commands = input.split("$ ")
        .filter(|s| ! s.is_empty())
        .map(Command::parse)
        .chain(std::iter::repeat(Command::ReturnToParent)); // Make sure we go back up towards the root directory

    let mut dir_sizes: Vec<usize> = Vec::new();
    super::lib::traverse_directory(commands, | dir_size | {
        dir_sizes.push(dir_size);
    });

    let total_disk_space: usize = 70000000;
    let required_free_space: usize = 30000000;
    let total_used_space = dir_sizes.iter().max().unwrap();
    let current_free_space = total_disk_space - total_used_space;
    let required_additional_space = required_free_space - current_free_space;
    let size_of_dir_to_delete = *dir_sizes
        .iter()
        .filter(|&&size| size >= required_additional_space)
        .min()
        .unwrap();

    size_of_dir_to_delete
}

pub mod tests {
    use criterion::Criterion;
    use std::path::Path;

    #[test]
    fn verify_example() {
        assert_eq!(super::solve(super::super::INPUT_EXAMPLE), 24933642);
    }

    #[test]
    fn verify_solution() {
        assert_eq!(super::solve(super::super::INPUT), 8319096);
    }

    pub fn benchmark(c: &mut Criterion) {
        let path = Path::new(file!());
        let day = path.parent().unwrap().to_str().unwrap();
        let solution = path.file_stem().unwrap().to_str().unwrap();
        let id = day.to_owned() + "_" + solution; // dayX_partY_solveZ
        c.bench_function( &id, |b| b.iter(|| super::solve(super::super::INPUT)));
    }
}