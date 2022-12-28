
use itertools::Itertools;

#[derive(Clone)]
pub enum Command {
    GoToRoot,
    ReturnToParent,
    ExploreChild,
    ListFileSizes(usize)
}

impl Command {
    pub fn parse(command_with_output: &str) -> Command {
        let (command, response) = command_with_output.split_once("\r\n").unwrap();
        let exec = command.split(' ').collect_vec();

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
                let (size, _) = entry.split_once(' ').unwrap();
                if size == "dir" {
                    None
                } else {
                    Some(size.parse::<usize>().unwrap())
                }
            }).sum();
            return Command::ListFileSizes(file_sizes);
        }

        unreachable!()
    }
}

pub fn traverse_directory(commands: impl Iterator<Item = Command>, mut directory_callback: impl FnMut(usize)) {
    let mut directory_sizes = Vec::new();
    for command in commands {
        match command {
            Command::GoToRoot => {
                // Reset directory stack
                directory_sizes = vec![0usize; 1];
            }
            Command::ReturnToParent => {
                // Calculate final size of directory
                let dir_size = directory_sizes.pop().unwrap();
                directory_callback(dir_size);

                if !directory_sizes.is_empty() {
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

    if !directory_sizes.is_empty() {
        panic!("The commands didn't take us back to the root!")
    }
}