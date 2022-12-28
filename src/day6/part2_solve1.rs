
pub fn solve(input: &str) -> i32 {
    let mut windows = input
        .as_bytes()
        .windows(14)
        .enumerate();

    let somm = windows.find(|(_, window)| {
        let mut found_cache = [false; 26];
        for c in window.iter() {
            let i = *c as usize - 'a' as usize;
            if found_cache[i] {
                return false; // Duplicate char found in this window
            }
            found_cache[i] = true;
        }
        true
    }).unwrap();

    (somm.0 as i32) + 14
}

pub mod tests {
    use criterion::Criterion;
    use std::path::Path;

    #[test]
    fn verify_examples() {
        assert_eq!(super::solve("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(super::solve("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(super::solve("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(super::solve("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(super::solve("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }

    #[test]
    fn verify_solution() {
        assert_eq!(super::solve(super::super::INPUT), 3534);
    }

    pub fn benchmark(c: &mut Criterion) {
        let path = Path::new(file!());
        let day = path.parent().unwrap().to_str().unwrap();
        let solution = path.file_stem().unwrap().to_str().unwrap();
        let id = day.to_owned() + "_" + solution; // dayX_partY_solveZ
        c.bench_function( &id, |b| b.iter(|| super::solve(super::super::INPUT)));
    }
}