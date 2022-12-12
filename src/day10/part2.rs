use super::lib::get_cycle_iterator;

pub fn solve(input: &str) -> String {
    let crt_width = 40;
    let crt_height = 6;
    let num_pixels = crt_width * crt_height;

    let mut output = String::with_capacity(crt_width * crt_height + (crt_height * 2));
    let cycles = get_cycle_iterator(input);
    for (cycle, register) in cycles.skip(1).take(num_pixels).enumerate() {
        let sprite_x = register;
        let scan_x = (cycle % crt_width) as i32;

        if (sprite_x - scan_x).abs() <= 1 {
            // Scanning within 3-pixel sprite centered on `sprite_x`
            output.push('#');
        } else {
            // Scanning outside of sprite
            output.push('.');
        }

        // Move to next line
        if (scan_x % 40) == crt_width as i32 - 1 {
            output.push_str("\r\n")
        }
    }

    //println!("{}", output);
    return output;
}

pub mod tests {
    use criterion::Criterion;
    use std::path::Path;

    #[test]
    fn verify_example() {
        assert_eq!(super::solve(super::super::INPUT_EXAMPLE), super::super::OUTPUT_EXAMPLE);
    }

    #[test]
    fn verify_solution() {
        assert_eq!(super::solve(super::super::INPUT), super::super::OUTPUT);
    }

    pub fn benchmark(c: &mut Criterion) {
        let path = Path::new(file!());
        let day = path.parent().unwrap().to_str().unwrap();
        let solution = path.file_stem().unwrap().to_str().unwrap();
        let id = day.to_owned() + "_" + solution; // dayX_partY_solveZ
        c.bench_function( &id, |b| b.iter(|| super::solve(super::super::INPUT)));
    }
}