use itertools::Itertools;

fn index_wraparound(i: i64, array_size: i64) -> i64 {
    ((i % array_size) + array_size) % array_size
}

fn mix(encrypted: &Vec<i32>, key: i32, rounds: i32) -> Vec<i64> {
    let mut decrypted = encrypted
        .iter()
        .map(|&n| n as i64 * key as i64)
        .enumerate()
        .collect_vec();

    for _ in 0..rounds {
        for original_i in 0..encrypted.len() {
            let current_i = decrypted.iter().position(|(i, _)| *i == original_i).unwrap();
            let (_, n) = decrypted[current_i];
            let destination_i = index_wraparound(current_i as i64 + n, decrypted.len() as i64 - 1) as usize;
    
            // Remove n from old position
            decrypted.remove(current_i);
    
            // Insert n in new position
            decrypted.insert(destination_i, (original_i, n));
        }
    }

    decrypted.iter().map(|(_, n)| *n).collect_vec()
}

pub fn solve(input: &str) -> i64 {
    let encrypted: Vec<i32> = input
        .lines()
        .map(|n| n.parse().unwrap())
        .collect_vec();

    // "Mix" the file and decrypt it
    let decrypted = mix(&encrypted, 811589153, 10);

    // Find coordinates
    let zero_i = decrypted.iter().position(|x| *x == 0).unwrap() as i64;
    let a_i = index_wraparound(zero_i + 1000, decrypted.len() as i64) as usize;
    let b_i = index_wraparound(zero_i + 2000, decrypted.len() as i64) as usize;
    let c_i = index_wraparound(zero_i + 3000, decrypted.len() as i64) as usize;
    decrypted[a_i] + decrypted[b_i] + decrypted[c_i]
}

pub mod tests {
    use criterion::Criterion;
    use std::path::Path;

    #[test]
    fn verify_example() {
        assert_eq!(super::solve(super::super::INPUT_EXAMPLE), 1623178306);
    }

    #[test]
    fn verify_solution() {
        assert_eq!(super::solve(super::super::INPUT), 1428396909280);
    }

    pub fn benchmark(c: &mut Criterion) {
        let path = Path::new(file!());
        let day = path.parent().unwrap().to_str().unwrap();
        let solution = path.file_stem().unwrap().to_str().unwrap();
        let id = day.to_owned() + "_" + solution; // dayX_partY_solveZ
        c.bench_function( &id, |b| b.iter(|| super::solve(super::super::INPUT)));
    }
}