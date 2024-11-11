use std::collections::HashSet;
use std::fs;

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/21.txt").unwrap());
    println!("Day 21:");
    println!("{}", solve_part_a(&input, 64));
    println!("{}", solve_part_b(&input, 26501365, false));
}

fn parse_input(input: String) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn solve_part_a(grid: &Vec<Vec<char>>, steps: usize) -> usize {
    count_paths(grid, steps, false).0
}

fn _solve_part_b_slow(grid: &Vec<Vec<char>>, steps: usize) -> usize {
    count_paths(grid, steps, true).0
}

fn count_paths(
    grid: &Vec<Vec<char>>,
    steps: usize,
    infinite_grid: bool,
) -> (usize, Vec<(i32, usize)>) {
    let n_rows = grid.len() as i32;
    let n_cols = grid[0].len() as i32;

    let mut visit_idx: Vec<Vec<Vec<usize>>> = vec![vec![vec![]; n_cols as usize]; n_rows as usize];

    // find positions of S
    let mut start_pos: (i32, i32) = (0, 0);
    for i in 0..n_rows as usize {
        for j in 0..n_cols as usize {
            if grid[i][j] == 'S' {
                start_pos = (i as i32, j as i32);
            }
        }
    }

    let even_visits: &mut HashSet<(i32, i32)> = &mut HashSet::new();
    let odd_visits: &mut HashSet<(i32, i32)> = &mut HashSet::new();

    let mut frontier = vec![start_pos];
    for step in 0..steps {
        // println!(
        //     "Step {}, front: {}, odd: {} even: {}",
        //     step,
        //     frontier.len(),
        //     odd_visits.len(),
        //     even_visits.len()
        // );
        let mut new_frontier = vec![];
        for (rfront, cfront) in frontier {
            for (rdiff, cdiff) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
                if !infinite_grid
                    && ((rdiff == -1 && rfront == 0)
                        || (rdiff == 1 && rfront == n_rows - 1)
                        || (cdiff == -1 && cfront == 0)
                        || (cdiff == 1 && cfront == n_cols - 1))
                {
                    // position is out of bounds
                    continue;
                }
                let nei_pos = (rfront + rdiff, cfront + cdiff);
                // println!(
                //     "{} {} {} {}",
                //     rfront + rdiff,
                //     (rfront + rdiff).rem_euclid(n_rows),
                //     cfront + cdiff,
                //     (cfront + cdiff).rem_euclid(n_cols)
                // );

                if grid[nei_pos.0.rem_euclid(n_rows) as usize]
                    [nei_pos.1.rem_euclid(n_cols) as usize]
                    == '#'
                {
                    // hit a rock
                    continue;
                }

                let visits_set: &mut HashSet<(i32, i32)> = if step % 2 == 0 {
                    odd_visits
                } else {
                    even_visits
                };

                if !visits_set.contains(&nei_pos) {
                    visits_set.insert(nei_pos);
                    new_frontier.push(nei_pos);
                    visit_idx[nei_pos.0.rem_euclid(n_rows) as usize]
                        [nei_pos.1.rem_euclid(n_cols) as usize]
                        .push(step);
                }
            }
        }
        frontier = new_frontier;
    }
    // println!("{:?}", visit_idx);
    // for row in visit_idx {
    //     println!("{:?}", row);
    // }

    if steps % 2 == 0 {
        let visited_rows_set: HashSet<i32> = even_visits.iter().map(|(r, _)| *r).collect();
        let mut visited_rows: Vec<i32> = visited_rows_set.into_iter().collect();
        visited_rows.sort();
        let visits_counter: Vec<(i32, usize)> = visited_rows
            .iter()
            .map(|row| (*row, even_visits.iter().filter(|(r, _)| r == row).count()))
            .collect();

        // for row in visited_rows.iter() {
        //     let visits = even_visits.iter().filter(|(r, _)| r == row).count();
        //     println!("{row} - {visits}")
        // }
        (even_visits.len(), visits_counter)
    } else {
        let visited_rows_set: HashSet<i32> = odd_visits.iter().map(|(r, _)| *r).collect();
        let mut visited_rows: Vec<i32> = visited_rows_set.into_iter().collect();
        visited_rows.sort();
        let visits_counter: Vec<(i32, usize)> = visited_rows
            .iter()
            .map(|row| (*row, odd_visits.iter().filter(|(r, _)| r == row).count()))
            .collect();

        // for row in visited_rows.iter() {
        //     let visits = odd_visits.iter().filter(|(r, _)| r == row).count();
        //     // if *row == 190 {
        //     //     odd_visits
        //     //         .iter()
        //     //         .filter(|(r, _)| r == row)
        //     //         .for_each(|(r, c)| {
        //     //             println!(
        //     //                 "{r} {c} {}",
        //     //                 grid[(*r).rem_euclid(n_rows) as usize]
        //     //                     [(*c).rem_euclid(n_cols) as usize]
        //     //             )
        //     //         });
        //     // }
        //     println!("{row} - {visits}");
        // }
        (odd_visits.len(), visits_counter)
    }
}

fn solve_part_b(grid: &Vec<Vec<char>>, steps: i32, debug: bool) -> i64 {
    let n_rows = grid.len() as i32;
    let n_cols = grid[0].len() as i32;
    let extended_n_rows = 3 * n_rows;
    let extended_n_cols = 3 * n_cols;

    // let mut extended_grid: Vec<Vec<char>> = vec![vec!['@'; 3*n_cols as usize]; 3*n_rows as usize];
    let extended_grid: Vec<Vec<char>> = (0..extended_n_rows)
        .map(|i| grid[i.rem_euclid(n_rows) as usize].repeat(3))
        .collect();
    assert_eq!(extended_grid.len(), extended_n_rows as usize);
    assert_eq!(extended_grid[0].len(), extended_n_cols as usize);

    // let mut visit_idx: Vec<Vec<i32>> = vec![vec![-1; 3 * n_cols as usize]; 3 * n_rows as usize];

    // find positions of S
    let mut start_pos: (i32, i32) = (0, 0);
    for i in 0..n_rows as usize {
        for j in 0..n_cols as usize {
            if grid[i][j] == 'S' {
                start_pos = (i as i32, j as i32);
            }
        }
    }

    // find distances of each element of grid to the origin
    let mut visit_idx: Vec<Vec<i32>> =
        vec![vec![-1; extended_n_cols as usize]; extended_n_rows as usize];
    let visits_set: &mut HashSet<(i32, i32)> = &mut HashSet::new();

    let extended_start_pos = (n_rows + start_pos.0, n_cols + start_pos.1);
    visits_set.insert(extended_start_pos);
    let mut frontier = vec![extended_start_pos];
    visit_idx[extended_start_pos.0 as usize][extended_start_pos.1 as usize] = 0;
    let mut step = 0;
    while !frontier.is_empty() {
        step += 1;
        let mut new_frontier = vec![];
        for (rfront, cfront) in frontier {
            for (rdiff, cdiff) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
                if (rdiff == -1 && rfront == 0)
                    || (rdiff == 1 && rfront == extended_n_rows - 1)
                    || (cdiff == -1 && cfront == 0)
                    || (cdiff == 1 && cfront == extended_n_cols - 1)
                {
                    // position is out of bounds
                    continue;
                }
                let nei_pos = (rfront + rdiff, cfront + cdiff);
                // println!(
                //     "{} {} {} {}",
                //     rfront + rdiff,
                //     (rfront + rdiff).rem_euclid(n_rows),
                //     cfront + cdiff,
                //     (cfront + cdiff).rem_euclid(n_cols)
                // );

                if extended_grid[nei_pos.0 as usize][nei_pos.1 as usize] == '#' {
                    // hit a rock
                    continue;
                }

                if !visits_set.contains(&nei_pos) {
                    visits_set.insert(nei_pos);
                    new_frontier.push(nei_pos);
                    visit_idx[nei_pos.0 as usize][nei_pos.1 as usize] = step;
                }
            }
        }
        frontier = new_frontier;
    }

    // if debug {
    let reference_scores = if debug {
        count_paths(grid, steps as usize, true).1
    } else {
        vec![]
    };
    // } else {
    //     let reference_scores: Vec<i32> = vec![];
    // }
    let mut total_steps: i64 = 0;
    let initial_parity = if steps % 2 == 1 { 1 } else { -1 };
    for i in (extended_start_pos.0 - steps)..=(extended_start_pos.0 + steps) {
        if i % 100_000 == 0 {
            let un_step = i + extended_start_pos.0 + steps;
            println!("{}/{}", un_step, steps * 2);
        }
        let row = if i < 0 {
            i.rem_euclid(n_rows)
        } else if i >= extended_n_rows {
            2 * n_rows + (i.rem_euclid(n_rows))
        } else {
            i
        } as usize;

        // let row = i.rem_euclid(extended_n_rows) as usize;
        // how many steps were spent going up or down
        let steps_offset = (extended_start_pos.0 - i).abs();
        // get "row" parity
        let parity = if steps_offset <= (n_rows - 1) / 2 {
            (1 + initial_parity) / 2
        } else {
            (1 + (initial_parity
                * (-1 as i32).pow(((steps_offset - 1 - ((n_rows - 1) / 2)) / n_rows) as u32)))
                / 2
        };
        // println!(
        //     "{:?} {:?}",
        //     parity,
        //     (1 + ((steps_offset - ((n_rows - 1) / 2)) / n_rows) as u32)
        // );
        // how many horizontal steps we are considering here
        let steps_left = steps - steps_offset;

        let whole_repetitions = ((steps_left - ((n_cols - 1) / 2)) / n_cols) as i64;
        let partial_steps = if whole_repetitions > 0 {
            (steps_left - ((n_cols - 1) / 2)).rem_euclid(n_cols)
        } else {
            steps_left
        };
        // go left and right
        let steps_half_line_right: i64 = ((n_cols + start_pos.1) as usize..(2 * n_cols) as usize)
            .enumerate()
            .map(|(_si, c)| {
                if visit_idx[row][c] == -1
                    // || visit_idx[row][c] > (steps_offset + si) as i32
                    || visit_idx[row][c] % 2 != parity
                {
                    0
                } else {
                    1
                }
            })
            .sum();
        let steps_half_line_left: i64 = (n_cols as usize..(n_cols + start_pos.1) as usize)
            .rev()
            .enumerate()
            .map(|(_si, c)| {
                if visit_idx[row][c] == -1
                    // || visit_idx[row][c] > (steps_offset + si) as i32
                    || visit_idx[row][c] % 2 != parity
                {
                    0
                } else {
                    1
                }
            })
            .sum();
        let steps_block_right_par: i64 = ((2 * n_cols) as usize..(3 * n_cols) as usize)
            .enumerate()
            .map(|(_si, c)| {
                if visit_idx[row][c] == -1
                    // || visit_idx[row][c] > (n_cols / 2 + steps_offset + si as i32)
                    || visit_idx[row][c] % 2 != parity
                {
                    0
                } else {
                    1
                }
            })
            .sum();
        let steps_block_right_not_par: i64 = ((2 * n_cols) as usize..(3 * n_cols) as usize)
            .enumerate()
            .map(|(_si, c)| {
                if visit_idx[row][c] == -1
                    // || visit_idx[row][c] > (n_cols / 2 + steps_offset + si as i32)
                    || visit_idx[row][c] % 2 == parity
                {
                    0
                } else {
                    1
                }
            })
            .sum();
        let steps_block_left_par: i64 = (0..n_cols as usize)
            .rev()
            .enumerate()
            .map(|(_si, c)| {
                if visit_idx[row][c] == -1
                    // || visit_idx[row][c] > (n_cols / 2 + steps_offset + si as i32)
                    || visit_idx[row][c] % 2 != parity
                {
                    0
                } else {
                    1
                }
            })
            .sum();
        let steps_block_left_not_par: i64 = (0..n_cols as usize)
            .rev()
            .enumerate()
            .map(|(_si, c)| {
                if visit_idx[row][c] == -1
                    // || visit_idx[row][c] > (n_cols / 2 + steps_offset + si as i32)
                    || visit_idx[row][c] % 2 == parity
                {
                    0
                } else {
                    1
                }
            })
            .sum();

        let partial_line_parity =
            (1 + ((2 * parity) - 1) * (-1 as i32).pow(whole_repetitions as u32)) / 2;
        let partial_line_right: i64 = if whole_repetitions > 0 {
            ((2 * n_cols) as usize..(2 * n_cols + partial_steps) as usize)
                .enumerate()
                .map(|(_si, c)| {
                    if visit_idx[row][c] == -1
                        // || visit_idx[row][c] > (n_cols / 2 + steps_offset + si as i32)
                        || visit_idx[row][c] % 2 != partial_line_parity
                    {
                        0
                    } else {
                        1
                    }
                })
                .sum()
        } else {
            ((n_cols + start_pos.1 + 1) as usize..=(n_cols + start_pos.1 + partial_steps) as usize)
                .enumerate()
                .map(|(_si, c)| {
                    // println!(
                    //     "{row} c:{c} si:{si} vidx:{} par:{parity}",
                    //     visit_idx[row][c]
                    // );
                    // if row == 321 || row == 323 {
                    //     println!(
                    //         "{row} {c} {steps_offset} {si} {} {}",
                    //         visit_idx[row][c], extended_grid[row][c]
                    //     )
                    // }
                    if visit_idx[row][c] == -1
                        || visit_idx[row][c] > steps // (steps_offset + 1 + si as i32)
                        || visit_idx[row][c] % 2 != parity
                    {
                        0
                    } else {
                        1
                    }
                })
                .sum()
        };
        let partial_line_left: i64 = if whole_repetitions > 0 {
            ((n_cols - partial_steps) as usize..n_cols as usize)
                .rev()
                .enumerate()
                .map(|(_si, c)| {
                    if visit_idx[row][c] == -1
                        // || visit_idx[row][c] > (n_cols / 2 + steps_offset + si as i32)
                        || visit_idx[row][c] % 2 != partial_line_parity
                    {
                        0
                    } else {
                        1
                    }
                })
                .sum()
        } else {
            ((n_cols + start_pos.1 - partial_steps) as usize..=(n_cols + start_pos.1) as usize)
                .rev()
                .enumerate()
                .map(|(_si, c)| {
                    // println!(
                    //     "{row} c:{c} si:{si} vidx:{} par:{parity}",
                    //     visit_idx[row][c]
                    // );
                    // if row == 321 || row == 323 {
                    //     println!(
                    //         "{row} {c} {steps_offset} {si} {} {}",
                    //         visit_idx[row][c], extended_grid[row][c]
                    //     )
                    // }
                    if visit_idx[row][c] == -1
                        || visit_idx[row][c] > steps //(steps_offset + 0 + si as i32)
                        || visit_idx[row][c] % 2 != parity
                    {
                        0
                    } else {
                        1
                    }
                })
                .sum()
        };
        let turn_steps = if whole_repetitions > 0 {
            steps_half_line_right + steps_half_line_left
        } else {
            0
        } + (((whole_repetitions / 2) + (whole_repetitions % 2))
            * (steps_block_right_par + steps_block_left_par))
            + (whole_repetitions / 2) * (steps_block_right_not_par + steps_block_left_not_par)
            + (partial_line_right + partial_line_left);
        total_steps += turn_steps;
        if debug {
            let (ref_row, ref_turn_steps) =
                reference_scores[(i - extended_start_pos.0 + steps) as usize];
            if ref_turn_steps != turn_steps as usize {
                println!(
                    "{} {} {} - {}",
                    i,
                    i - extended_start_pos.0,
                    ref_row,
                    ref_turn_steps
                );
                println!("{i} {row} wr:{whole_repetitions} sl:{steps_left} so:{steps_offset} par:{parity} shlr:{steps_half_line_right} shll:{steps_half_line_left} sbrp:{steps_block_right_par} sbrn: {steps_block_right_not_par} sblp:{steps_block_left_par} sbln: {steps_block_left_not_par} plr:{partial_line_right} pll:{partial_line_left} ts:{turn_steps}");
            }
        }
    }

    total_steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input, 6), 16);
        assert_eq!(_solve_part_b_slow(&input, 6), 16);
        // assert_eq!(_solve_part_b_slow(&input, 50), 1594);
        // assert_eq!(_solve_part_b_slow(&input, 500), 167004);
        let input = parse_input(fs::read_to_string("inputs/21.txt").unwrap());
        assert_eq!(
            _solve_part_b_slow(&input, 2) as i64,
            solve_part_b(&input, 2, true)
        );
        assert_eq!(
            _solve_part_b_slow(&input, 3) as i64,
            solve_part_b(&input, 3, true)
        );
        // slower, but valid tests. Turning them off to speed up things
        // assert_eq!(
        //     _solve_part_b_slow(&input, 65) as i64,
        //     solve_part_b(&input, 65, true)
        // );
        // assert_eq!(
        //     _solve_part_b_slow(&input, 66) as i64,
        //     solve_part_b(&input, 66, true)
        // );
        // assert_eq!(
        //     _solve_part_b_slow(&input, 101) as i64,
        //     solve_part_b(&input, 101, true)
        // );
        // assert_eq!(
        //     _solve_part_b_slow(&input, 131) as i64,
        //     solve_part_b(&input, 131, true)
        // );
        // assert_eq!(
        //     _solve_part_b_slow(&input, 589) as i64,
        //     solve_part_b(&input, 589, false)
        // );
        // assert_eq!(
        //     _solve_part_b_slow(&input, 458) as i64,
        //     solve_part_b(&input, 458, false)
        // );
        // assert_eq!(
        //     _solve_part_b_slow(&input, 327) as i64,
        //     solve_part_b(&input, 327, false)
        // );
        // assert_eq!(
        //     _solve_part_b_slow(&input, 197) as i64,
        //     solve_part_b(&input, 197, true)
        // );
        // assert_eq!(
        //     _solve_part_b_slow(&input, 196) as i64,
        //     solve_part_b(&input, 196, true)
        // );
    }
}
// got 618261429072038 after running for 80 min - too low! :(
