
pub fn solve(input: &str) -> i32 {
    let mut iter = input.lines();
    let mut largest = 0;
    loop {
        let elf: i32 = iter.by_ref()
            .take_while(|line| !line.is_empty())
            .map(|line| line.parse::<i32>().unwrap())
            .sum();

        if elf == 0 {
            break;
        }

        largest = std::cmp::max(largest, elf);
    }
    return largest;
}


pub mod tests {
    use super::*;

    #[test]
    fn verify_solution() {
        assert_eq!(solve(super::super::INPUT), 65912);
    }

    pub fn benchmark() {
        solve(super::super::INPUT);
    }
}