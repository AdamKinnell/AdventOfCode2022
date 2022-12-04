use std::collections::HashSet;


fn find_duplicate_item(rucksack: &str) -> char {
    let (left, right) = rucksack.split_at(rucksack.len() / 2);
    let left_items : HashSet<char> = HashSet::from_iter(left.chars());
    let right_items : HashSet<char> = HashSet::from_iter(right.chars());
    return *left_items.intersection(&right_items).next().unwrap();
}

fn find_item_value(item: char) -> i32 {
    match item {
        'a'..='z' => (item as i32 - 'a' as i32) + 1,
        'A'..='Z' => (item as i32 - 'A' as i32) + 27,
        _ => unreachable!()
    }
}

pub fn solve(input: &str) -> i32 {
    input
        .lines()
        .map(|rucksack| find_duplicate_item(rucksack) )
        .map(|item| find_item_value(item))
        .sum()
}

pub mod tests {
    use criterion::Criterion;
    use std::path::Path;

    #[test]
    fn verify_example() {
        assert_eq!(super::solve(super::super::INPUT_EXAMPLE), 157);
    }

    #[test]
    fn verify_solution() {
        assert_eq!(super::solve(super::super::INPUT), 8072);
    }

    pub fn benchmark(c: &mut Criterion) {
        let path = Path::new(file!());
        let day = path.parent().unwrap().to_str().unwrap();
        let solution = path.file_stem().unwrap().to_str().unwrap();
        let id = day.to_owned() + "_" + solution; // dayX_partY_solveZ
        c.bench_function( &id, |b| b.iter(|| super::solve(super::super::INPUT)));
    }
}   