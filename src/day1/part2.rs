
pub fn solve(input: &str) -> i32 {
    let mut elves: Vec<i32> = input.split("\r\n\r\n")
        .map(|elf| {
            elf
            .lines()
            .filter(|x| !x.is_empty())
            .map(|line| line.parse::<i32>().unwrap())
            .sum()
        }).collect();
    
    elves.sort();

    return elves.iter().rev().take(3).sum();
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn verify_solution() {
        assert_eq!(solve(super::super::INPUT), 195625);
    }
}