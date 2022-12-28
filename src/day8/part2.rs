use grid::*;
use itertools::Itertools;

pub struct Tree {
    pub height: u8
}

pub fn parse_trees(input: &str) -> Grid<Tree> {
    let width = input.find("\r\n").unwrap();
    let trees = input
        .bytes()
        .filter_map(|x| {
            if (b'0'..=b'9').contains(&x) {
                let tree = Tree {height: x - b'0'};
                Some(tree)
            } else {
                None
            }
    }).collect_vec();

    Grid::from_vec(trees, width)
}

fn calc_scenic_score(trees: &Grid<Tree>, tree_x: usize, tree_y: usize) -> i32 {
    let my_height = trees[tree_x][tree_y].height;

    // Check Right
    let mut view_right = 0;
    for x in (tree_x+1)..trees.cols() {
        view_right += 1;
        if trees[x][tree_y].height >= my_height {
            break;
        }
    }

    // Check Down
    let mut view_down = 0;
    for y in (tree_y+1)..trees.rows() {
        view_down += 1;
        if trees[tree_x][y].height >= my_height {
            break;
        }
    }

    // Check Left
    let mut view_left = 0;
    for x in (0..=((tree_x as i32)-1)).rev() {
        view_left += 1;
        if trees[x as usize][tree_y].height >= my_height {
            break;
        }
    }

    // Check Up
    let mut view_up = 0;
    for y in (0..=((tree_y as i32)-1)).rev() {
        view_up += 1;
        if trees[tree_x][y as usize].height >= my_height {
            break;
        }
    }
    
    view_right * view_down * view_left * view_up
}

pub fn solve(input: &str) -> i32 {
    let trees = &parse_trees(input);

    let mut highest = 0;
    for x in 0..trees.cols() {
        for y in 0..trees.rows() {
            let score = calc_scenic_score(trees, x, y);
            if score > highest {
                highest = score;
            }
            //print!(" {} ", score)
        }
        //println!("")
    }

    highest
}

pub mod tests {
    use criterion::Criterion;
    use std::path::Path;


    #[test]
    fn verify_scenic_score_a() {
        let trees = super::parse_trees(
        "000\r\n\
         010\r\n\
         000\r\n"
        );

        assert_eq!(super::calc_scenic_score(&trees, 1, 1), 1)
    }

    #[test]
    fn verify_scenic_score_example() {
        let trees = super::parse_trees(super::super::INPUT_EXAMPLE);
        assert_eq!(super::calc_scenic_score(&trees, 3, 2), 8)
    }


    #[test]
    fn verify_example() {
        assert_eq!(super::solve(super::super::INPUT_EXAMPLE), 8);
    }

    #[test]
    fn verify_solution() {
        assert_eq!(super::solve(super::super::INPUT), 315495);
    }

    pub fn benchmark(c: &mut Criterion) {
        let path = Path::new(file!());
        let day = path.parent().unwrap().to_str().unwrap();
        let solution = path.file_stem().unwrap().to_str().unwrap();
        let id = day.to_owned() + "_" + solution; // dayX_partY_solveZ
        c.bench_function( &id, |b| b.iter(|| super::solve(super::super::INPUT)));
    }
}                                                                             