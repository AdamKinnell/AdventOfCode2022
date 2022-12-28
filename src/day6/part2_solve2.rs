
fn find_first_unique_window(input: &str, window_size: i32) -> Option<usize> {
    let chars = input.as_bytes();

    let mut duplicate_chars = 0;
    let mut chars_in_window = [0; 26];
    for (i, c) in chars.iter().enumerate() {
        let charcode = *c as usize - 'a' as usize;
        
        // Mark char as being in this window
        chars_in_window[charcode] += 1;
        if chars_in_window[charcode] > 1 {
            // If we have more than one instance of this char, then we just added a duplicate.
            duplicate_chars += 1;
        }

        // As the window slides, unmark the char that was aged out
        let aged_out_charindex = i as i32 - window_size;
        if aged_out_charindex >= 0 {
            let aged_out_charcode = chars[aged_out_charindex as usize] as usize - 'a' as usize;
            chars_in_window[aged_out_charcode] -= 1;
            if chars_in_window[aged_out_charcode] >= 1 {
                // If we still have an instance of this char, then we just removed a duplicate.
                duplicate_chars -= 1;
            }
            assert!(duplicate_chars >= 0);
            assert!(chars_in_window[aged_out_charcode] >= 0);
        }

        // Check if we have a full window with no duplicates
        if aged_out_charindex >= -1 && duplicate_chars == 0 {
            return Some(i);
        }
    }

    None
}

pub fn solve(input: &str) -> Option<usize> {
    find_first_unique_window(input, 14).map(|x| x + 1)
}

pub mod tests {
    use criterion::Criterion;
    use std::path::Path;

    #[test]
    fn verify_function() {
        assert_eq!(super::find_first_unique_window("aaaa", 4), None);
        assert_eq!(super::find_first_unique_window("aabcd", 4), Some(4));
    }

    #[test]
    fn verify_examples() {
        assert_eq!(super::solve("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), Some(19));
        assert_eq!(super::solve("bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(23));
        assert_eq!(super::solve("nppdvjthqldpwncqszvftbrmjlhg"), Some(23));
        assert_eq!(super::solve("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Some(29));
        assert_eq!(super::solve("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Some(26));
    }

    #[test]
    fn verify_solution() {
        assert_eq!(super::solve(super::super::INPUT), Some(3534));
    }

    pub fn benchmark(c: &mut Criterion) {
        let path = Path::new(file!());
        let day = path.parent().unwrap().to_str().unwrap();
        let solution = path.file_stem().unwrap().to_str().unwrap();
        let id = day.to_owned() + "_" + solution; // dayX_partY_solveZ
        c.bench_function( &id, |b| b.iter(|| super::solve(super::super::INPUT)));
    }
}