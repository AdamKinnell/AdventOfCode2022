use std::str::Lines;
use itertools::{Itertools, Chunk};

fn find_item_value(item: char) -> i32 {
    match item {
        'a'..='z' => (item as i32 - 'a' as i32) + 1,
        'A'..='Z' => (item as i32 - 'A' as i32) + 27,
        _ => unreachable!()
    }
}

fn find_badge_item(group: Chunk<Lines>) -> char {
    let group = group
    .sorted_by(|x,y| x.len().cmp(&y.len()))
    .collect_vec();
    
    // Hardcoded to groups of size 3
    assert_eq!(group.len(), 3);
    for a in group[0].chars() {
        for b in group[1].chars() {
            if a == b {
                for c in group[2].chars() {
                    if b == c {
                        return c;
                    }
                }
            }
        }
    }
    unreachable!()
}

pub fn solve(input: &str) -> i32 {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .map(find_badge_item)
        .map(find_item_value)
        .sum()
}

pub mod tests {
    use criterion::Criterion;
    use std::path::Path;

    #[test]
    fn verify_example() {
        assert_eq!(super::solve(super::super::INPUT_EXAMPLE), 70);
    }

    #[test]
    fn verify_solution() {
        assert_eq!(super::solve(super::super::INPUT), 2567);
    }

    pub fn benchmark(c: &mut Criterion) {
        let path = Path::new(file!());
        let day = path.parent().unwrap().to_str().unwrap();
        let solution = path.file_stem().unwrap().to_str().unwrap();
        let id = day.to_owned() + "_" + solution; // dayX_partY_solveZ
        c.bench_function( &id, |b| b.iter(|| super::solve(super::super::INPUT)));
    }
}