use std::convert::identity;


fn get_cycle_iterator<'a>(input: &'a str) -> impl Iterator<Item = i32> + 'a {
    let cycles = input.lines().flat_map(|line| {
        if line.starts_with("addx") {
            // addx - 2 cycles, but the updated value isn't present until the second cycle
            let amount = line.split(" ").nth(1).unwrap().parse::<i32>().unwrap();
            return [Some(0), Some(amount)];
        } else {
            // noop - 1 cycle
            return [Some(0), None];
        }
    }).filter_map(identity) // Hack to remove the second element emitted from a no-op
    .scan(1, |acc, x| {
        *acc += x;
        return Some(*acc);
    });

    return [1i32, 1i32].into_iter().chain(cycles);
}

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