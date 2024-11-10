use std::collections::HashSet;
use std::fs;

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/21.txt").unwrap());
    println!("Day 21:");
    println!("{}", solve_part_a(&input, 64));
    println!("{}", solve_part_b(&input, 26501365));
}

fn parse_input(input: String) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn solve_part_a(grid: &Vec<Vec<char>>, steps: usize) -> usize {
    count_paths(grid, steps, false)
}

fn solve_part_b_slow(grid: &Vec<Vec<char>>, steps: usize) -> usize {
    count_paths(grid, steps, true)
}

fn count_paths(grid: &Vec<Vec<char>>, steps: usize, infinite_grid: bool) -> usize {
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
        println!("{visited_rows_set:?}");
        let mut visited_rows: Vec<i32> = visited_rows_set.into_iter().collect();
        visited_rows.sort();

        for row in visited_rows.iter() {
            let visits = even_visits.iter().filter(|(r, _)| r == row).count();
            println!("{row} - {visits}")
        }
        even_visits.len()
    } else {
        let visited_rows_set: HashSet<i32> = odd_visits.iter().map(|(r, _)| *r).collect();
        let mut visited_rows: Vec<i32> = visited_rows_set.into_iter().collect();
        visited_rows.sort();

        for row in visited_rows.iter() {
            let visits = odd_visits.iter().filter(|(r, _)| r == row).count();
            if *row == 190 {
                odd_visits
                    .iter()
                    .filter(|(r, _)| r == row)
                    .for_each(|(r, c)| {
                        println!(
                            "{r} {c} {}",
                            grid[(*r).rem_euclid(n_rows) as usize]
                                [(*c).rem_euclid(n_cols) as usize]
                        )
                    });
            }
            println!("{row} - {visits}");
        }
        odd_visits.len()
    }
}

fn solve_part_b_shortcut(grid: &Vec<Vec<char>>, steps: usize) -> usize {
    // compute size of main square
    // steps = 26501365;
    // n_rows = n_cols = 130
    let n_rows = grid.len() as i32;
    let n_cols = grid[0].len() as i32;

    let whole_repetitions = steps as i32 / (n_rows / 2);
    let square_size = n_rows * (steps as i32 / n_rows);
    println!("{steps}, {n_rows}, {n_cols}");
    todo!()

    // compute size of nodes outside main square
}

fn solve_part_b(grid: &Vec<Vec<char>>, steps: i32) -> i64 {
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

    let mut total_steps: i64 = 0;
    let initial_parity = if steps % 2 == 1 { 1 } else { -1 };
    for i in (extended_start_pos.0 - steps)..=(extended_start_pos.0 + steps) {
        let row = i.rem_euclid(extended_n_rows) as usize;
        // how many steps were spent going up or down
        let steps_offset = (extended_start_pos.0 - i).abs();
        let parity = if steps_offset <= (n_rows - 1) / 2 {
            (1 + initial_parity) / 2
        } else {
            (1 + (initial_parity
                * (-1 as i32).pow(0 + ((steps_offset - 1 - ((n_rows - 1) / 2)) / n_rows) as u32)))
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
            .map(|(si, c)| {
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
            .map(|(si, c)| {
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
        let steps_block_right: i64 = ((2 * n_cols) as usize..(3 * n_cols) as usize)
            .enumerate()
            .map(|(si, c)| {
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
        let steps_block_left: i64 = (0..n_cols as usize)
            .rev()
            .enumerate()
            .map(|(si, c)| {
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
        let partial_line_right: i64 = if whole_repetitions > 0 {
            ((2 * n_cols) as usize..(2 * n_cols + partial_steps) as usize)
                .enumerate()
                .map(|(si, c)| {
                    if visit_idx[row][c] == -1
                        || visit_idx[row][c] > (n_cols / 2 + steps_offset + si as i32)
                        || visit_idx[row][c] % 2 != parity
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
                .map(|(si, c)| {
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
                .map(|(si, c)| {
                    if visit_idx[row][c] == -1
                        || visit_idx[row][c] > (n_cols / 2 + steps_offset + si as i32)
                        || visit_idx[row][c] % 2 != parity
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
                .map(|(si, c)| {
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
        } + whole_repetitions * (steps_block_right + steps_block_left)
            + (partial_line_right + partial_line_left);
        total_steps += turn_steps;
        println!("{i} {row} wr:{whole_repetitions} sl:{steps_left} so:{steps_offset} par:{parity} shlr:{steps_half_line_right} shll:{steps_half_line_left} sbr:{steps_block_right} sbl:{steps_block_left} plr:{partial_line_right} pll:{partial_line_left} ts:{turn_steps}");
    }
    println!("{extended_n_rows}");

    total_steps
}

fn solve_part_b_hard(grid: &Vec<Vec<char>>, steps: i32) -> i64 {
    let n_rows = grid.len() as i32;
    let n_cols = grid[0].len() as i32;

    // let mut extended_grid: Vec<Vec<char>> = vec![vec!['@'; 3*n_cols as usize]; 3*n_rows as usize];
    let extended_grid: Vec<Vec<char>> = (0..n_rows)
        .map(|i| grid[i.rem_euclid(n_rows) as usize].repeat(3))
        .collect();
    assert_eq!(extended_grid.len(), 1 * n_rows as usize);
    assert_eq!(extended_grid[0].len(), 3 * n_cols as usize);

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

    let mut total_steps: i64 = 0;
    let parity: usize = (steps % 2) as usize;
    for i in 0..n_rows {
        // println!("x {i} {total_steps} {n_rows}");
        // how many horizontal steps are left, when discounted the vertical ones used to reach this row
        let steps_minus_vert = steps - (start_pos.0 - i).abs();
        // print!("{steps_minus_vert} ");
        if steps_minus_vert < 0 {
            continue;
        }

        // TODO: optimize this to remove this loop
        for t in (0..=steps_minus_vert as usize).step_by(n_rows as usize) {
            // how many steps were spent going up or down
            let steps_offset = (start_pos.0 - i).abs() as usize + t;
            // how many horizontal steps we are considering here
            let steps_left = steps_minus_vert - t as i32;

            let whole_repetitions = ((steps_left - ((n_cols - 1) / 2)) / n_cols) as i64;
            let partial_steps = if whole_repetitions > 0 {
                (steps_left - ((n_cols - 1) / 2)).rem_euclid(n_cols)
            } else {
                steps_left
            };
            // go left and right
            let steps_half_line_right: i64 = extended_grid[i as usize]
                [(n_cols + start_pos.1) as usize..(2 * n_cols) as usize]
                .iter()
                .enumerate()
                .map(|(si, c)| {
                    if *c == '#' || ((steps_offset + si) % 2 != parity) {
                        0
                    } else {
                        1
                    }
                })
                .sum();
            let steps_half_line_left: i64 = extended_grid[i as usize]
                [n_cols as usize..(n_cols + start_pos.1) as usize]
                .iter()
                .rev()
                .enumerate()
                .map(|(si, c)| {
                    if *c == '#' || ((steps_offset + si) % 2 != parity) {
                        0
                    } else {
                        1
                    }
                })
                .sum();
            let steps_block_right: i64 = extended_grid[i as usize]
                [(2 * n_cols) as usize..(3 * n_cols) as usize]
                .iter()
                .enumerate()
                .map(|(si, c)| {
                    if *c == '#' || ((steps_offset + si) % 2 != parity) {
                        0
                    } else {
                        1
                    }
                })
                .sum();
            let steps_block_left: i64 = extended_grid[i as usize][0..n_cols as usize]
                .iter()
                .rev()
                .enumerate()
                .map(|(si, c)| {
                    if *c == '#' || ((steps_offset + si) % 2 != parity) {
                        0
                    } else {
                        1
                    }
                })
                .sum();
            let partial_line_right: i64 = if whole_repetitions > 0 {
                extended_grid[i as usize]
                    [(2 * n_cols) as usize..(2 * n_cols + partial_steps) as usize]
                    .iter()
                    .enumerate()
                    .map(|(si, c)| {
                        if *c == '#' || ((steps_offset + si) % 2 != parity) {
                            0
                        } else {
                            1
                        }
                    })
                    .sum()
            } else {
                extended_grid[i as usize][(n_cols + start_pos.1 + 1) as usize
                    ..=(n_cols + start_pos.1 + partial_steps) as usize]
                    .iter()
                    .enumerate()
                    .map(|(si, c)| {
                        if *c == '#' || ((steps_offset + si) % 2 == parity) {
                            0
                        } else {
                            1
                        }
                    })
                    .sum()
            };
            let partial_line_left: i64 = if whole_repetitions > 0 {
                extended_grid[i as usize][(n_cols - partial_steps) as usize..n_cols as usize]
                    .iter()
                    .rev()
                    .enumerate()
                    .map(|(si, c)| {
                        if *c == '#' || ((steps_offset + si) % 2 != parity) {
                            0
                        } else {
                            1
                        }
                    })
                    .sum()
            } else {
                extended_grid[i as usize][(n_cols + start_pos.1 - partial_steps) as usize
                    ..=(n_cols + start_pos.1) as usize]
                    .iter()
                    .rev()
                    .enumerate()
                    .map(|(si, c)| {
                        if *c == '#' || ((steps_offset + si) % 2 != parity) {
                            0
                        } else {
                            1
                        }
                    })
                    .sum()
            };
            let turn_steps = if whole_repetitions > 0 {
                steps_half_line_right + steps_half_line_left - 1
            } else {
                0
            } + whole_repetitions * (steps_block_right + steps_block_left)
                + (partial_line_right + partial_line_left);
            total_steps += turn_steps;
            println!("{i} {t} wr:{whole_repetitions} sl:{steps_left} so:{steps_offset} par:{parity} shlr:{steps_half_line_right} shll:{steps_half_line_left} sbr:{steps_block_right} sbl:{steps_block_left} plr:{partial_line_right} pll:{partial_line_left} ts:{turn_steps}");
        }
    }

    total_steps

    // let visits_set: &mut HashSet<(i32, i32)> = &mut HashSet::new();

    // let mut frontier = vec![(n_rows + start_pos.0, n_cols + start_pos.1)];
    // let mut step = 0;
    // while !frontier.is_empty() {
    //     step += 1;
    //     let mut new_frontier = vec![];
    //     for (rfront, cfront) in frontier {
    //         for (rdiff, cdiff) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
    //             if (rdiff == -1 && rfront == 0)
    //                 || (rdiff == 1 && rfront == n_rows - 1)
    //                 || (cdiff == -1 && cfront == 0)
    //                 || (cdiff == 1 && cfront == n_cols - 1)
    //             {
    //                 // position is out of bounds
    //                 continue;
    //             }
    //             let nei_pos = (rfront + rdiff, cfront + cdiff);

    //             if extended_grid[nei_pos.0 as usize][nei_pos.1 as usize] == '#' {
    //                 // hit a rock
    //                 continue;
    //             }

    //             if !visits_set.contains(&nei_pos) {
    //                 visits_set.insert(nei_pos);
    //                 new_frontier.push(nei_pos);
    //                 visit_idx[nei_pos.0 as usize][nei_pos.1 as usize] = step;
    //             }
    //         }
    //     }
    //     frontier = new_frontier;
    // }
    // // println!("{:?}", visit_idx);
    // for row in &visit_idx {
    //     println!("{:?}", row);
    // }
    // // println!("{:?}", trans_grid);
    // // mid-down
    // todo!();
}
fn solve_part_b_tests(grid: &Vec<Vec<char>>, steps: usize) -> usize {
    let n_rows = grid.len() as i32;
    let n_cols = grid[0].len() as i32;

    let mut visit_idx: Vec<Vec<i32>> = vec![vec![-1; n_cols as usize]; n_rows as usize];

    // find positions of S
    let mut start_pos: (i32, i32) = (0, 0);
    for i in 0..n_rows as usize {
        for j in 0..n_cols as usize {
            if grid[i][j] == 'S' {
                start_pos = (i as i32, j as i32);
            }
        }
    }

    // SE
    // let mut trans_grid: Vec<Vec<char>> = vec![vec!['@'; n_cols as usize]; n_rows as usize];
    // for i in 0..n_rows as usize {
    //     for j in 0..n_cols as usize {
    //         // trans_grid[i][j] = grid[(start_pos.0 + i as i32).rem_euclid(n_rows) as usize]
    //         //     [(start_pos.1 + j as i32).rem_euclid(n_cols) as usize]
    //         trans_grid[i][j] = grid[i][j]
    //     }
    // }

    let visits_set: &mut HashSet<(i32, i32)> = &mut HashSet::new();

    let mut frontier = vec![start_pos];
    let mut step = 0;
    while !frontier.is_empty() {
        step += 1;
        let mut new_frontier = vec![];
        for (rfront, cfront) in frontier {
            for (rdiff, cdiff) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
                if (rdiff == -1 && rfront == 0)
                    || (rdiff == 1 && rfront == n_rows - 1)
                    || (cdiff == -1 && cfront == 0)
                    || (cdiff == 1 && cfront == n_cols - 1)
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

                if grid[nei_pos.0 as usize][nei_pos.1 as usize] == '#' {
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
    // println!("{:?}", visit_idx);
    for row in &visit_idx {
        println!("{:?}", row);
    }
    for row in 0..n_rows as usize {
        // assert_ne!(trans_grid[row][0], '#');
        // assert_ne!(trans_grid[row][0], '@');
        assert_ne!(
            grid[row][(start_pos.1 + row as i32).rem_euclid(n_cols) as usize],
            '#',
            "failed at {row}"
        );
        assert_ne!(
            grid[row][(start_pos.1 - row as i32).rem_euclid(n_cols) as usize],
            '#',
            "failed at {row}"
        );
        let jjj = (start_pos.1 + row as i32).rem_euclid(n_cols) as usize;
        assert_eq!(
            visit_idx[row][(start_pos.1 + row as i32).rem_euclid(n_cols - 1) as usize],
            65,
            "failed at {row} {jjj} {start_pos:?}"
        );
        assert_eq!(
            visit_idx[row][(start_pos.1 - row as i32).rem_euclid(n_cols - 1) as usize],
            65,
            "failed at {row}"
        );
        // assert_ne!(row[(n_cols - 1) as usize], '#');
        // assert_ne!(row[(n_cols - 1) as usize], '@');
    }
    // println!("{:?}", trans_grid);
    // mid-down
    todo!();
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
        assert_eq!(solve_part_b_slow(&input, 6), 16);
        assert_eq!(solve_part_b_slow(&input, 50), 1594);
        // assert_eq!(solve_part_b(&input, 100), 6536);
        assert_eq!(solve_part_b_slow(&input, 500), 167004);
        // assert_eq!(solve_part_b(&input, 1000), 668697);
        // assert_eq!(solve_part_b(&input, 5000), 16733044);
        let input = parse_input(fs::read_to_string("inputs/21.txt").unwrap());
        assert_eq!(
            solve_part_b_slow(&input, 196) as i64,
            solve_part_b(&input, 196)
        );
        // 196 == 65 + 131
        // 0-130 131-196 196-261 262-392
        //
        // 197 - 64
        // 196
        // 65 - 197
        assert_eq!(solve_part_b_slow(&input, 2) as i64, solve_part_b(&input, 2));
        assert_eq!(solve_part_b_slow(&input, 3) as i64, solve_part_b(&input, 3));
        assert_eq!(
            solve_part_b_slow(&input, 65) as i64,
            solve_part_b(&input, 65)
        );
        assert_eq!(
            solve_part_b_slow(&input, 66) as i64,
            solve_part_b(&input, 66)
        );
        assert_eq!(
            solve_part_b_slow(&input, 101) as i64,
            solve_part_b(&input, 101)
        );
        assert_eq!(
            solve_part_b_slow(&input, 131) as i64,
            solve_part_b(&input, 131)
        );
    }
}
