use std::fs;

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/14.txt").unwrap());
    println!("Day 14:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn solve_part_a(grid: &Vec<Vec<char>>) -> usize {
    let n_rows = grid.len();
    let n_cols = grid[0].len();
    let mut rock_pos = vec![n_rows + 1; n_cols];

    let mut load = 0;
    for i in 0..n_rows {
        for j in 0..n_cols {
            match grid[i][j] {
                'O' => {
                    load += rock_pos[j] - 1;
                    rock_pos[j] -= 1;
                }
                '#' => rock_pos[j] = n_rows - i,
                _ => {}
            }
        }
    }
    load
}

fn solve_part_b(input: &Vec<Vec<char>>) -> usize {
    solve_part_b_brent(input)
}

fn solve_part_b_brent(input: &Vec<Vec<char>>) -> usize {
    let mut loads = vec![];
    let mut power = 1;
    let mut cycle_length = 1;
    let mut tortoise = input.clone();
    let mut hare = input.clone();
    loads.push(get_load(&hare));
    cycle(&mut hare);
    loads.push(get_load(&hare));

    while tortoise != hare {
        if power == cycle_length {
            tortoise = hare.clone();
            power *= 2;
            cycle_length = 0;
        }
        cycle(&mut hare);
        loads.push(get_load(&hare));
        cycle_length += 1;
    }

    let mut tortoise = input.clone();
    let mut hare = input.clone();
    for _ in 0..cycle_length {
        cycle(&mut hare)
    }

    let mut cycle_start = 0;
    while tortoise != hare {
        cycle(&mut tortoise);
        cycle(&mut hare);
        cycle_start += 1;
    }

    // println!("{loads:?}");
    // println!("Cycle start: {cycle_start} cycle_length: {cycle_length}");
    let ans_pos = (1_000_000_000 - cycle_start) % cycle_length;
    loads[cycle_start + ans_pos]
}

fn _solve_part_b_floyd(input: &Vec<Vec<char>>) -> usize {
    let mut loads = vec![];
    let mut tortoise = input.clone();
    cycle(&mut tortoise);
    let mut hare = input.clone();
    loads.push(get_load(&hare));
    cycle(&mut hare);
    loads.push(get_load(&hare));
    cycle(&mut hare);
    loads.push(get_load(&hare));

    while tortoise != hare {
        cycle(&mut tortoise);
        cycle(&mut hare);
        loads.push(get_load(&hare));
        cycle(&mut hare);
        loads.push(get_load(&hare));
    }

    let mut cycle_start = 0;
    let mut tortoise = input.clone();
    while tortoise != hare {
        cycle(&mut tortoise);
        cycle(&mut hare);
        loads.push(get_load(&hare));
        cycle_start += 1;
    }

    let mut cycle_length = 1;
    let mut hare = tortoise.clone();
    cycle(&mut hare);
    while tortoise != hare {
        cycle(&mut hare);
        cycle_length += 1;
    }
    // println!("{loads:?}");
    // println!("Cycle start: {cycle_start} cycle_length: {cycle_length}");
    // println!(
    //     "A: {:?}",
    //     loads[cycle_start..cycle_start + cycle_length + 1].to_vec()
    // );
    // println!(
    //     "B: {:?}",
    //     loads[cycle_start + cycle_length..cycle_start + 2 * cycle_length + 1].to_vec()
    // );

    let ans_pos = (1_000_000_000 - cycle_start) % cycle_length;
    loads[cycle_start + ans_pos]
}

fn get_load(m: &Vec<Vec<char>>) -> usize {
    let n_rows = m.len();
    let n_cols = m[0].len();

    let mut load = 0;
    for i in 0..n_rows {
        for j in 0..n_cols {
            if m[i][j] == 'O' {
                load += n_rows - i;
            }
        }
    }
    load
}

fn cycle(grid: &mut Vec<Vec<char>>) {
    let n_rows = grid.len();
    let n_cols = grid[0].len();

    // north
    let mut rock_pos = vec![0; n_cols];
    for i in 0..n_rows {
        for j in 0..n_cols {
            match grid[i][j] {
                'O' => {
                    grid[i][j] = '.';
                    grid[rock_pos[j]][j] = 'O';
                    rock_pos[j] += 1;
                }
                '#' => rock_pos[j] = i + 1,
                _ => {}
            }
        }
    }
    // west
    let mut rock_pos = vec![0; n_rows];
    for j in 0..n_cols {
        for i in 0..n_rows {
            match grid[i][j] {
                'O' => {
                    grid[i][j] = '.';
                    grid[i][rock_pos[i]] = 'O';
                    rock_pos[i] += 1;
                }
                '#' => rock_pos[i] = j + 1,
                _ => {}
            }
        }
    }
    // south
    let mut rock_pos = vec![n_rows - 1; n_cols];
    for i in (0..n_rows).rev() {
        for j in (0..n_cols).rev() {
            match grid[i][j] {
                'O' => {
                    grid[i][j] = '.';
                    grid[rock_pos[j]][j] = 'O';
                    rock_pos[j] = if rock_pos[j] > 0 { rock_pos[j] - 1 } else { 0 };
                }
                '#' => rock_pos[j] = if i > 0 { i - 1 } else { 0 },
                _ => {}
            }
        }
    }
    // east
    let mut rock_pos = vec![n_rows - 1; n_rows];
    for j in (0..n_cols).rev() {
        for i in (0..n_rows).rev() {
            match grid[i][j] {
                'O' => {
                    grid[i][j] = '.';
                    grid[i][rock_pos[i]] = 'O';
                    rock_pos[i] = if rock_pos[i] > 0 { rock_pos[i] - 1 } else { 0 };
                }
                '#' => rock_pos[i] = if j > 0 { j - 1 } else { 0 },
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input), 136);
        assert_eq!(solve_part_b(&input), 64);
    }
}
