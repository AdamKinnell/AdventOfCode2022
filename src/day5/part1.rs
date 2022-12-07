use itertools::Itertools;

fn parse_crate_positions(input: &str) -> Vec::<Vec<char>> {
    let mut iter = input.lines().rev(); // Look at lines in reverse order
    let last_line = iter.next().unwrap();
    let num_stacks = last_line.split_ascii_whitespace().count();
    let mut crates = (0..num_stacks).map(|_| Vec::<char>::new()).collect_vec();

    iter.for_each(|row| {
        let chars = row.as_bytes();
        for stack in 0..num_stacks {
            let char = chars[1 + (stack * 4)] as char;
            if char != ' ' {
                crates[stack].push(char)
            }
        }
    });

    return crates;
}

fn print_crates(crates: &Vec<Vec<char>>) {
    println!("========");
    crates.iter().for_each(|stack| println!("{}", stack.iter().join("")));
    println!("========");
    println!("Printed on side. Rotate 90 degrees counter-clockwise to view.")
}

fn apply_instruction(instruction: &str, crates: &mut Vec<Vec<char>>) {
    let (_, count, _, from, _, to ) = instruction
        .split_ascii_whitespace()
        .collect_tuple()
        .unwrap();

    let count = count.parse().unwrap();
    let from: usize = from.parse().unwrap();
    let to: usize = to.parse().unwrap();
    for _ in 0..count  {
        let elem: char = crates[from - 1].pop().unwrap();
        crates[to - 1].push(elem);
    }
}

fn find_top_of_stacks(crates: &Vec<Vec<char>>) -> String {
    return crates
        .iter()
        .map(|stack| stack.last().unwrap())
        .join("");
}

pub fn solve(input: &str) -> String {
    let (crates_diagram, instructions) = input.split_once("\r\n\r\n").unwrap();
    let crates = &mut parse_crate_positions(crates_diagram);
    //print_crates(&crates);

    instructions
        .lines()
        .for_each(|instruction| {
            apply_instruction(instruction, crates);
            //print_crates(&crates);
        });

    return find_top_of_stacks(&crates);
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