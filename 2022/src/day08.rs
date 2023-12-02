use std::{fs, usize};

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/input08.in").unwrap());
    println!("Day 8:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input))
}

fn parse_input(input: String) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| line.bytes().map(|b| (b - b'0') as i32).collect())
        .collect()
}

fn solve_part_a(buildings: &Vec<Vec<i32>>) -> usize {
    let n_rows = buildings.len();
    let n_cols = buildings[0].len();
    let mut counter: Vec<Vec<usize>> = (0..n_rows).map(|_| vec![0; n_cols]).collect();

    for (range1, range2, rows_first) in [
        (
            (0..n_rows).collect::<Vec<usize>>(),
            (0..n_cols).collect::<Vec<usize>>(),
            true,
        ), // left
        ((0..n_rows).collect(), (0..n_cols).rev().collect(), true), // right
        ((0..n_cols).collect(), (0..n_rows).collect(), false),      // down
        ((0..n_cols).collect(), (0..n_rows).rev().collect(), false), // up
    ] {
        for i in &range1 {
            let mut max_height = -1;
            for j in &range2 {
                let ii = if rows_first { *i } else { *j };
                let jj = if rows_first { *j } else { *i };
                if buildings[ii][jj] > max_height {
                    counter[ii][jj] = 1;
                    max_height = buildings[ii][jj];
                }
            }
        }
    }

    // count all cells
    counter.iter().map(|row| row.iter().sum::<usize>()).sum()
}

fn solve_part_b(buildings: &Vec<Vec<i32>>) -> i32 {
    // O(n) solution for part 2. To avoid rewriting code 4 times (one for each
    // direction), I used the rows_first and reversed flags, which made the code
    // very messy and ugly. There might be better ways to refactor it, but I'm
    // happy with the algorithm's complexity.
    let n_rows = buildings.len();
    let n_cols = buildings[0].len();
    let mut counter: Vec<Vec<i32>> = (0..n_rows).map(|_| vec![1; n_cols]).collect();
    // erase counter for edges
    for i in 0..n_rows {
        for j in 0..n_cols {
            if i == 0 || i == n_rows - 1 || j == 0 || j == n_cols - 1 {
                counter[i][j] = 0;
            }
        }
    }

    for (range1, range2, rows_first, reversed) in [
        (
            (1..n_rows - 1).collect::<Vec<usize>>(),
            (1..n_cols - 1).collect::<Vec<usize>>(),
            true,
            false,
        ), // left -> right
        (
            (1..n_rows - 1).collect(),
            (1..n_cols - 1).rev().collect(),
            true,
            true,
        ), // right -> left
        (
            (1..n_cols - 1).collect(),
            (1..n_rows - 1).collect(),
            false,
            false,
        ), // up -> down
        (
            (1..n_cols - 1).collect(),
            (1..n_rows - 1).rev().collect(),
            false,
            true,
        ), // down -> up
    ] {
        for i in &range1 {
            let last = if rows_first { n_rows } else { n_cols };
            let start_idx = if !reversed { 0 } else { last - 1 };
            let mut last_height_idx: usize = start_idx;
            let mut dir_counter: Vec<usize> = vec![0; last];
            for j in &range2 {
                let ii = if rows_first { *i } else { *j };
                let jj = if rows_first { *j } else { *i };
                let jjj = if rows_first { jj } else { ii };
                if buildings[ii][jj]
                    <= if rows_first {
                        buildings[ii][last_height_idx]
                    } else {
                        buildings[last_height_idx][jj]
                    }
                {
                    dir_counter[jjj] = 1;
                } else {
                    // Although there is a while loop here, this algorithm is
                    // still O(n), (with n = n_cols*n_rows), as last_height_idx
                    // works as a stack, and this stack never push the an index
                    // to its top more than once - for each idx, after it's
                    // added, it either is removed or remains in the stack until
                    // the end of the execution.
                    while last_height_idx != start_idx
                        && if rows_first {
                            buildings[ii][last_height_idx]
                        } else {
                            buildings[last_height_idx][jj]
                        } < buildings[ii][jj]
                    {
                        if !reversed {
                            last_height_idx -= dir_counter[last_height_idx];
                        } else {
                            last_height_idx += dir_counter[last_height_idx];
                        }
                    }
                    if !reversed {
                        dir_counter[jjj] = jjj - last_height_idx;
                    } else {
                        dir_counter[jjj] = last_height_idx - jjj;
                    }
                }
                last_height_idx = jjj;
                counter[ii][jj] *= dir_counter[jjj] as i32;
            }
        }
    }

    // count all cells
    *counter
        .iter()
        .map(|row| row.iter().max().unwrap())
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = r#"30373
25512
65332
33549
35390"#;

        let input = parse_input(sample.to_string());

        assert_eq!(21, solve_part_a(&input));
        assert_eq!(8, solve_part_b(&input));
    }
}
