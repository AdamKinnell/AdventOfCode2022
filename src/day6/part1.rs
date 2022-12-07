use itertools::Itertools;

pub fn solve(input: &str) -> i32 {
    let chars = input.chars();

    let sopm = chars
        .into_iter()
        .tuple_windows()
        .enumerate()
        .find(|(_, (a,b,c,d))| {
            return a != b && a != c && a != d &&
                             b != c && b != d &&
                                       c != d;
        }).unwrap();

    return (sopm.0 as i32) + 4;
}

pub mod tests {
    use criterion::Criterion;
    use std::path::Path;

    #[test]
    fn verify_examples() {
        assert_eq!(super::solve("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(super::solve("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(super::solve("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(super::solve("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(super::solve("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn verify_solution() {
        assert_eq!(super::solve(super::super::INPUT), 1093);
    }

    pub fn benchmark(c: &mut Criterion) {
        let path = Path::new(file!());
        let day = path.parent().unwrap().to_str().unwrap();
        let solution = path.file_stem().unwrap().to_str().unwrap();
        let id = day.to_owned() + "_" + solution; // dayX_partY_solveZ
        c.bench_function( &id, |b| b.iter(|| super::solve(super::super::INPUT)));
    }
}