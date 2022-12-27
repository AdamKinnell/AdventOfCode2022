
fn snafu_to_decimal(n: &str) -> i32 {

}

fn decimal_to_snafu(n: i32) -> String {
    return Strinf::
}

pub fn solve(input: &str) -> String {
    let sum = input
    .lines()
    .map(|line| snafu_to_decimal(line))
    .sum();

    return decimal_to_snafu(sum);
}

pub mod tests {
    use criterion::Criterion;
    use std::path::Path;

    #[test]
    fn verify_snafu_to_decimal() {
        assert_eq!(super::snafu_to_decimal("1"), 1);
        assert_eq!(super::snafu_to_decimal("2"), 2);
        assert_eq!(super::snafu_to_decimal("1="), 3);
        assert_eq!(super::snafu_to_decimal("1-"), 4);
        assert_eq!(super::snafu_to_decimal("10"), 5);
        assert_eq!(super::snafu_to_decimal("11"), 6);
        assert_eq!(super::snafu_to_decimal("12"), 7);
        assert_eq!(super::snafu_to_decimal("2="), 8);
        assert_eq!(super::snafu_to_decimal("2-"), 9);
        assert_eq!(super::snafu_to_decimal("20"), 10);
        assert_eq!(super::snafu_to_decimal("1=0"), 15);
        assert_eq!(super::snafu_to_decimal("1-0"), 20);
        assert_eq!(super::snafu_to_decimal("1=11-2"), 2022);
        assert_eq!(super::snafu_to_decimal("1-0---0"), 12345);
        assert_eq!(super::snafu_to_decimal("1121-1110-1=0"), 314159265);
    }

    #[test]
    fn verify_decimal_to_snafu() {
        assert_eq!(super::decimal_to_snafu(1), "1");
        assert_eq!(super::decimal_to_snafu(2), "2");
        assert_eq!(super::decimal_to_snafu(3), "1=");
        assert_eq!(super::decimal_to_snafu(4), "1-");
        assert_eq!(super::decimal_to_snafu(5), "10");
        assert_eq!(super::decimal_to_snafu(6), "11");
        assert_eq!(super::decimal_to_snafu(7), "12");
        assert_eq!(super::decimal_to_snafu(8), "2=");
        assert_eq!(super::decimal_to_snafu(9), "2-");
        assert_eq!(super::decimal_to_snafu(10), "20");
        assert_eq!(super::decimal_to_snafu(15), "1=0");
        assert_eq!(super::decimal_to_snafu(20), "1-0");
        assert_eq!(super::decimal_to_snafu(2022), "1=11-2");
        assert_eq!(super::decimal_to_snafu(12345), "1-0---0");
        assert_eq!(super::decimal_to_snafu(314159265), "1121-1110-1=0");
    }
    
    #[test]
    fn verify_example() {
        assert_eq!(super::solve(super::super::INPUT_EXAMPLE), "2=-1=0");
    }

    #[test]
    fn verify_solution() {
        assert_eq!(super::solve(super::super::INPUT), "A");
    }

    pub fn benchmark(c: &mut Criterion) {
        let path = Path::new(file!());
        let day = path.parent().unwrap().to_str().unwrap();
        let solution = path.file_stem().unwrap().to_str().unwrap();
        let id = day.to_owned() + "_" + solution; // dayX_partY_solveZ
        c.bench_function( &id, |b| b.iter(|| super::solve(super::super::INPUT)));
    }
}