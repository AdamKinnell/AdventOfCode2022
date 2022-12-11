use std::{vec, fs::DirEntry};

use itertools::Itertools;

#[derive(Clone)]
enum Command {
    GoToRoot,
    ReturnToParent,
    ExploreChild,
    ListFileSizes(usize)
}

impl Command {
    fn parse(command_with_output: &str) -> Command {
        let (command, response) = command_with_output.split_once("\r\n").unwrap();
        let exec = command.split(" ").collect_vec();

        if exec[0] == "cd" {
            if exec[1] == "/" {
                return Command::GoToRoot;
            } else if exec[1] == ".." {
                return Command::ReturnToParent;
            } else {
                return Command::ExploreChild;
            }
        } else if exec[0] == "ls" {
            let file_sizes: usize = response.lines().filter_map(|entry| {
                let (size, _) = entry.split_once(" ").unwrap();
                if size == "dir" {
                    return None;
                } else {
                    return Some(size.parse::<usize>().unwrap())
                }
            }).sum();
            return Command::ListFileSizes(file_sizes);
        }

        unreachable!()
    }
}

pub fn solve(input: &str) -> usize {
    let commands = input.split("$ ")
        .filter(|s| ! s.is_empty())
        .map(|command_line| Command::parse(command_line))
        .chain(std::iter::repeat(Command::ReturnToParent)); // Make sure we end at the root directory

    let mut sum_of_dirs_less_than_100000 = 0usize;
    let mut directory_sizes = Vec::new();
    for command in commands {
        match command {
            Command::GoToRoot => {
                // Reset directory stack
                directory_sizes = vec![0usize; 1];
            }
            Command::ReturnToParent => {
                // Calculate final sum of directory
                let dir_size = directory_sizes.pop().unwrap();
                if dir_size <= 100000 {
                    sum_of_dirs_less_than_100000 += dir_size;
                }

                if (directory_sizes.len() > 0) {
                    // Update parent with size of child directory
                    *directory_sizes.last_mut().unwrap() += dir_size;
                } else {
                    // We're already at the root
                    break;
                }
            }
            Command::ExploreChild => {
                // Start tracking a new child directory size
                directory_sizes.push(0usize);
            }
            Command::ListFileSizes(size) => {
                // Update this directory size
                *directory_sizes.last_mut().unwrap() += size;
            }
        }
    }

    return sum_of_dirs_less_than_100000;
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