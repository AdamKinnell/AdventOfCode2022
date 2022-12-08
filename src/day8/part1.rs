use grid::*;
use itertools::Itertools;

struct Tree {
    height: u8,
    is_visible: bool
}

fn parse_trees(input: &str) -> Grid<Tree> {
    let width = input.find("\r\n").unwrap();
    let trees = input
        .bytes()
        .filter_map(|x| {
            if x >= '0' as u8 && x <= '9' as u8 {
                let tree = Tree {height: x - '0' as u8, is_visible: false};
                return Some(tree);
            } else {
                return None;
            }
    }).collect_vec();

    return Grid::from_vec(trees, width);
}

// fn print_trees(trees: &Grid<Tree>) {
//     for x in 0..trees.cols() {
//         for y in 0..trees.rows() {
//             let char = trees[x][y].height + '0' as u8;
//             print!("{}", char as char);
//         }
//         println!("");
//     }
// }

// fn print_visibility(trees: &Grid<Tree>) {
//     for x in 0..trees.cols() {
//         for y in 0..trees.rows() {
//             if trees[x][y].is_visible {
//                 print!("{}",'V');
//             } else {
//                 print!("{}",'H');
//             }
//         }
//         println!("");
//     }
// }

pub fn solve(input: &str) -> usize {
    let mut trees = parse_trees(input);
    
    let mut visible_trees = 0;
    let (x_max, y_max) = trees.size();

    // From West
    for i_ray in 0..x_max {
        let mut highest = -1;
        for i_dist in 0..y_max {
            let mut tree = &mut trees[i_ray][i_dist];
            if tree.height as i32 > highest {
                // The tree is visible from this direction
                highest = tree.height as i32;
                if !tree.is_visible {
                    visible_trees += 1;
                    tree.is_visible = true;
                }
            }
        }
    }
    // println!("Visibility after rays from West:");
    // print_visibility(&trees);

    // From North
    for i_ray in 0..y_max {
        let mut highest = -1;
        for i_dist in 0..x_max {
            let mut tree = &mut trees[i_dist][i_ray];
            if tree.height as i32 > highest {
                // The tree is visible from this direction
                highest = tree.height as i32;
                if !tree.is_visible {
                    visible_trees += 1;
                    tree.is_visible = true;
                }
            }
        }
    }
    // println!("Visibility after rays from North:");
    // print_visibility(&trees);

    // From East
    for i_ray in 0..x_max {
        let mut highest = -1;
        for i_dist in (0..y_max).rev() {
            let mut tree = &mut trees[i_ray][i_dist];
            if tree.height as i32 > highest {
                // The tree is visible from this direction
                highest = tree.height as i32;
                if !tree.is_visible {
                    visible_trees += 1;
                    tree.is_visible = true;
                }
            }
        }
    }
    //println!("Visibility after rays from East:");
    //print_visibility(&trees);

    // From South
    for i_ray in 0..y_max {
        let mut highest = -1;
        for i_dist in (0..x_max).rev() {
            let mut tree = &mut trees[i_dist][i_ray];
            if tree.height as i32 > highest {
                // The tree is visible from this direction
                highest = tree.height as i32;
                if !tree.is_visible {
                    visible_trees += 1;
                    tree.is_visible = true;
                }
            }
        }
    }
    // println!("Visibility after rays from South:");
    // print_visibility(&trees);

    // print_trees(&trees);

    return visible_trees;
}

pub mod tests {
    use criterion::Criterion;
    use std::path::Path;

    #[test]
    fn verify_inners_are_hidden() {
        assert_eq!(super::solve(
            "00300\r\n\
             03230\r\n\
             32423\r\n\
             03230\r\n\
             00300\r\n"
        ), 21);
    }

    #[test]
    fn verify_all_visible_b() {
        assert_eq!(super::solve(
            "00000\r\n\
             01110\r\n\
             01210\r\n\
             01110\r\n\
             00000\r\n"
        ), 25);
    }

    #[test]
    fn verify_simple() {
        assert_eq!(super::solve(
            "000\r\n\
             010\r\n\
             000\r\n"
        ), 9);
    }

    #[test]
    fn verify_example() {
        assert_eq!(super::solve(super::super::INPUT_EXAMPLE), 21);
    }

    #[test]
    fn verify_solution() {
        assert_eq!(super::solve(super::super::INPUT), 1812);
    }

    pub fn benchmark(c: &mut Criterion) {
        let path = Path::new(file!());
        let day = path.parent().unwrap().to_str().unwrap();
        let solution = path.file_stem().unwrap().to_str().unwrap();
        let id = day.to_owned() + "_" + solution; // dayX_partY_solveZ
        c.bench_function( &id, |b| b.iter(|| super::solve(super::super::INPUT)));
    }
}