// use std::{collections::HashMap, fs};
use std::{collections::HashSet, fs};

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
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
    let rows = input.len();
    let cols = input[0].len();
    let mut start_pos = (0, 0);
    for i in 0..rows {
        for j in 0..cols {
            if input[i][j] == '^' {
                start_pos = (i, j);
            }
        }
    }

    guard_walk(input, start_pos).0.len()
}

fn solve_part_b(input: &Vec<Vec<char>>) -> usize {
    let rows = input.len();
    let cols = input[0].len();
    let mut start_pos = (0, 0);
    for i in 0..rows {
        for j in 0..cols {
            if input[i][j] == '^' {
                start_pos = (i, j);
            }
        }
    }

    let mut canonical_path = guard_walk(input, start_pos).0;
    canonical_path.remove(&(start_pos.0, start_pos.1));

    let mut grid = input.clone();
    canonical_path
        .iter()
        .filter(|(i, j)| {
            grid[*i][*j] = '#';
            let has_cycle = guard_walk(&grid, start_pos).1;
            grid[*i][*j] = '.';
            has_cycle
        })
        .count()
}

fn guard_walk(grid: &Vec<Vec<char>>, cur_pos: (usize, usize)) -> (HashSet<(usize, usize)>, bool) {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    let mut visits: HashSet<(usize, usize, Dir)> =
        HashSet::with_capacity((rows * cols * 4) as usize);
    let mut cur_dir = Dir::Up;
    let mut i = cur_pos.0 as i32;
    let mut j = cur_pos.1 as i32;
    while i >= 0 && i < rows && j >= 0 && j < cols {
        let ui = i as usize;
        let uj = j as usize;

        let cur_pos = (ui, uj, cur_dir.clone());
        if !visits.insert(cur_pos) {
            return (visits.iter().map(|(vi, vj, _)| (*vi, *vj)).collect(), true);
        }

        match cur_dir {
            Dir::Up => {
                if i > 0 && grid[ui - 1][uj] == '#' {
                    cur_dir = Dir::Right;
                } else {
                    i -= 1
                }
            }
            Dir::Right => {
                if j < cols - 1 && grid[ui][uj + 1] == '#' {
                    cur_dir = Dir::Down;
                } else {
                    j += 1
                }
            }
            Dir::Down => {
                if i < rows - 1 && grid[ui + 1][uj] == '#' {
                    cur_dir = Dir::Left;
                } else {
                    i += 1
                }
            }
            Dir::Left => {
                if j > 0 && grid[ui][uj - 1] == '#' {
                    cur_dir = Dir::Up;
                } else {
                    j -= 1
                }
            }
        }
    }
    (
        visits.iter().map(|(vi, vj, __)| (*vi, *vj)).collect(),
        false,
    )
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
