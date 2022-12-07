use itertools::Itertools;

pub fn parse_crate_positions(input: &str) -> Vec::<Vec<char>> {
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

pub fn print_crates(crates: &Vec<Vec<char>>) {
    println!("========");
    crates.iter().for_each(|stack| println!("{}", stack.iter().join("")));
    println!("========");
    println!("Printed on side. Rotate 90 degrees counter-clockwise to view.")
}

// Moves the specified number of crates so they are in reverse order on the new stack
pub fn apply_instruction(instruction: &str, crates: &mut Vec<Vec<char>>) {
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

// Moves the specified number of crates so they are in the same order on the new stack
pub fn apply_instruction2(instruction: &str, crates: &mut Vec<Vec<char>>) {
    let (_, count, _, from, _, to ) = instruction
        .split_ascii_whitespace()
        .collect_tuple()
        .unwrap();

    let count = count.parse().unwrap();
    let from: usize = from.parse().unwrap();
    let to: usize = to.parse().unwrap();

    let temp = &mut Vec::with_capacity(count);

    // Use a temp array, as Rust makes it hard to mutable borrow both stacks at once from `crates`.
    let from_stack_mut = &mut crates[from - 1];
    for _ in 0..count  {
        temp.push(from_stack_mut.pop().unwrap());
    }
    let to_stack_mut = &mut crates[to - 1];
    for _ in 0..count  {
        to_stack_mut.push(temp.pop().unwrap());
    }
}

pub fn find_top_of_stacks(crates: &Vec<Vec<char>>) -> String {
    return crates
        .iter()
        .map(|stack| stack.last().unwrap())
        .join("");
}