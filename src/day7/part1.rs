
struct Directory<'a> {
    name: &'a str,
    size: usize,
    parent: Option<&'a Directory<'a>>
}


pub fn solve(input: &str) -> i32 {
    // let mut root = Directory();
    // input.split("$ ").for_each(|command| {

    // });

    0
}

pub mod tests {
    use criterion::Criterion;
    use std::path::Path;

    #[test]
    fn verify_example() {
        assert_eq!(super::solve(super::super::INPUT_EXAMPLE), -1);
    }

    #[test]
    fn verify_solution() {
        assert_eq!(super::solve(super::super::INPUT), -1);
    }

    pub fn benchmark(c: &mut Criterion) {
        let path = Path::new(file!());
        let day = path.parent().unwrap().to_str().unwrap();
        let solution = path.file_stem().unwrap().to_str().unwrap();
        let id = day.to_owned() + "_" + solution; // dayX_partY_solveZ
        c.bench_function( &id, |b| b.iter(|| super::solve(super::super::INPUT)));
    }
}