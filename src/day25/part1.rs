use itertools::Itertools;

fn snafu_to_decimal(n: &str) -> i64 {
    let mut decimal = 0;

    // Convert to SNAFU number starting from left-most place
    let chars = n.chars().collect_vec();
    for i in 0..n.len() {
        let this_digit = chars[i];
        decimal *= 5;
        match this_digit {
            '2' => { decimal += 2; },
            '1' => { decimal += 1; },
            '0' => { decimal += 0; },
            '-' => { decimal -= 1; },
            '=' => { decimal -= 2; }
            _ => unreachable!()
        }
    }

    return decimal;
}

fn decimal_to_snafu(mut n: i64) -> String {
    let mut snafu = String::new();

    // Convert to SNAFU number starting from right-most place
    while n > 0 {
        let mut borrow = false;
        let this_digit = n % 5;
        match this_digit {
            0 => { snafu.push_str("0")},
            1 => { snafu.push_str("1")},
            2 => { snafu.push_str("2")},
            3 => { snafu.push_str("="); borrow = true}, // We have to borrow 1 from the next place
            4 => { snafu.push_str("-"); borrow = true}, // We have to borrow 2 from the next place
            _ => unreachable!()
        }
        n /= 5; // Go to next 5's place in the number
        if borrow {
            n += 1; // Give us an extra digit in the next place to borrow from
        }
    }

    // We built the SNAFU in reverse
    return snafu.chars().rev().collect::<String>();
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
        assert_eq!(super::solve(super::super::INPUT), "2-=0-=-2=111=220=100");
    }

    pub fn benchmark(c: &mut Criterion) {
        let path = Path::new(file!());
        let day = path.parent().unwrap().to_str().unwrap();
        let solution = path.file_stem().unwrap().to_str().unwrap();
        let id = day.to_owned() + "_" + solution; // dayX_partY_solveZ
        c.bench_function( &id, |b| b.iter(|| super::solve(super::super::INPUT)));
    }
}