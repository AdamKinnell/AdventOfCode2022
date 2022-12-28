

pub fn get_cycle_iterator<'a>(input: &'a str) -> impl Iterator<Item = i32> + 'a {
    let cycles = input.lines().flat_map(|line| {
        if line.starts_with("addx") {
            // addx - 2 cycles, but the updated value isn't present until the second cycle
            let amount = line.split(' ').nth(1).unwrap().parse::<i32>().unwrap();
            [Some(0), Some(amount)]
        } else {
            // noop - 1 cycle
            [Some(0), None]
        }
    }).flatten() // Hack to remove the second element emitted from a no-op
    .scan(1, |acc, x| {
        *acc += x;
        Some(*acc)
    });

    [1i32, 1i32].into_iter().chain(cycles)
}