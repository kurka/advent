use std::fs;

#[derive(Debug)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/06.txt").unwrap());
    println!("Day 06:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn solve_part_a(input: &Vec<Vec<char>>) -> usize {
    let rows = input.len() as i32;
    let cols = input[0].len() as i32;
    let mut grid = input.clone();

    let mut cur_pos = (0, 0);
    for i in 0..rows {
        for j in 0..cols {
            if grid[i as usize][j as usize] == '^' {
                cur_pos = (i, j);
            }
        }
    }

    let mut visits = 0;
    let mut cur_dir = Dir::Up;
    let (mut i, mut j) = cur_pos;
    while i >= 0 && i < rows && j >= 0 && j < cols {
        let ui = i as usize;
        let uj = j as usize;

        if grid[i as usize][j as usize] != 'X' {
            grid[i as usize][j as usize] = 'X';
            visits += 1
        }

        match cur_dir {
            Dir::Up => {
                if i > 0 && grid[ui - 1][uj] == '#' {
                    cur_dir = Dir::Right;
                    j += 1
                } else {
                    i -= 1
                }
            }
            Dir::Right => {
                if j < cols - 1 && grid[ui][uj + 1] == '#' {
                    cur_dir = Dir::Down;
                    i += 1
                } else {
                    j += 1
                }
            }
            Dir::Down => {
                if i < rows - 1 && grid[ui + 1][uj] == '#' {
                    cur_dir = Dir::Left;
                    j -= 1
                } else {
                    i += 1
                }
            }
            Dir::Left => {
                if j > 0 && grid[ui][uj - 1] == '#' {
                    cur_dir = Dir::Up;
                    i -= 1
                } else {
                    j -= 1
                }
            }
        }
    }

    visits
}

fn solve_part_b(input: &Vec<Vec<char>>) -> usize {
    let rows = input.len();
    let cols = input[0].len();
    let mut grid: Vec<Vec<usize>> = vec![vec![0; cols]; rows];

    let mut start_pos = (0, 0);
    for i in 0..rows {
        for j in 0..cols {
            if input[i][j] == '^' {
                start_pos = (i as i32, j as i32);
            }
        }
    }

    let mut visits = 0;

    let mut obstructions = 0;
    let mut cur_dir = Dir::Up;
    let (mut i, mut j) = start_pos;

    // choose one bit for each direction. This will allow storing multiple directions in a single cell
    let up_bit = 0b0001;
    let right_bit = 0b0010;
    let down_bit = 0b0100;
    let left_bit = 0b1000;
    while i >= 0 && i < rows as i32 && j >= 0 && j < cols as i32 {
        let ui = i as usize;
        let uj = j as usize;

        // if grid[i as usize][j as usize] != 'X' {
        //     grid[i as usize][j as usize] = 'X';
        //     visits += 1
        // }

        match cur_dir {
            Dir::Up => {
                grid[ui][uj] |= up_bit;
                if ui > 0 && input[ui - 1][uj] == '#' {
                    // grid[ui][uj] = 'F';
                    grid[ui][uj] |= right_bit;
                    cur_dir = Dir::Right;
                    j += 1
                } else {
                    // look right
                    if walk(input, &grid, i, j, right_bit, Dir::Right, rows, cols) {
                        obstructions += 1
                    }
                    i -= 1
                }
            }
            Dir::Right => {
                grid[ui][uj] |= right_bit;
                if uj < cols - 1 && input[ui][uj + 1] == '#' {
                    // grid[ui][uj] = '7';
                    grid[ui][uj] |= down_bit;
                    cur_dir = Dir::Down;
                    i += 1
                } else {
                    // look down
                    if walk(input, &grid, i, j, down_bit, Dir::Down, rows, cols) {
                        obstructions += 1
                    }
                    j += 1
                }
            }
            Dir::Down => {
                grid[ui][uj] |= down_bit;
                if ui < rows - 1 && input[ui + 1][uj] == '#' {
                    // grid[ui][uj] = 'J';
                    grid[ui][uj] |= left_bit;
                    cur_dir = Dir::Left;
                    j -= 1
                } else {
                    // look left
                    if walk(input, &grid, i, j, left_bit, Dir::Left, rows, cols) {
                        obstructions += 1
                    }
                    i += 1
                }
            }
            Dir::Left => {
                grid[ui][uj] |= 0b1000;
                if uj > 0 && input[ui][uj - 1] == '#' {
                    // grid[ui][uj] = 'L';
                    grid[ui][uj] |= up_bit;
                    cur_dir = Dir::Up;
                    i -= 1
                } else {
                    // look up
                    if walk(input, &grid, i, j, up_bit, Dir::Up, rows, cols) {
                        obstructions += 1
                    }
                    j -= 1
                }
            }
        }
    }
    for row in grid {
        println!("{row:x?}");
    }

    obstructions
}

fn walk(
    input: &Vec<Vec<char>>,
    grid: &Vec<Vec<usize>>,
    start_i: i32,
    start_j: i32,
    target: usize,
    start_dir: Dir,
    rows: usize,
    cols: usize,
) -> bool {
    // choose one bit for each direction. This will allow storing multiple directions in a single cell
    let up_bit = 0b0001;
    let right_bit = 0b0010;
    let down_bit = 0b0100;
    let left_bit = 0b1000;
    let mut i = start_i;
    let mut j = start_j;
    let mut cur_dir = start_dir;
    let mut target = target;
    while i >= 0 && i < rows as i32 && j >= 0 && j < cols as i32 {
        let ui = i as usize;
        let uj = j as usize;

        if grid[ui][uj] & target == target {
            return true;
        }

        match cur_dir {
            Dir::Up => {
                if ui > 0 && input[ui - 1][uj] == '#' {
                    cur_dir = Dir::Right;
                    target = right_bit;
                    j += 1
                } else {
                    i -= 1
                }
            }
            Dir::Right => {
                if uj < cols - 1 && input[ui][uj + 1] == '#' {
                    cur_dir = Dir::Down;
                    target = down_bit;
                    i += 1
                } else {
                    j += 1
                }
            }
            Dir::Down => {
                if ui < rows - 1 && input[ui + 1][uj] == '#' {
                    cur_dir = Dir::Left;
                    target = left_bit;
                    j -= 1
                } else {
                    i += 1
                }
            }
            Dir::Left => {
                if uj > 0 && input[ui][uj - 1] == '#' {
                    cur_dir = Dir::Up;
                    target = up_bit;
                    i -= 1
                } else {
                    j -= 1
                }
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input), 41);
        assert_eq!(solve_part_b(&input), 6);
    }
}

// ....#.....
// ....F>>>7#
// ....^...v.
// ..#.^...v.
// ..F>^>7#v.
// ..^.^.v.v.
// .#L<*<*<J.
// .F>>>>*7#.
// #L*<*<J*..
// ......#v..
